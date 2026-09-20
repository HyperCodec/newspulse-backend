use std::collections::{HashMap, HashSet};
use std::ops::Deref;

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum_extra::extract::Query;
use axum_openapi3::utoipa::openapi::{InfoBuilder, OpenApiBuilder};
use axum_openapi3::utoipa::*;
use axum_openapi3::{AddRoute, build_openapi, endpoint, utoipa};
use icalendar::{Calendar, Component, Event, EventLike};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use crate::AppState;
use crate::db::*;

pub struct InternalServerError;

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("internal server error".into())
            .unwrap()
    }
}

impl<T> From<T> for InternalServerError
where
    T: ToString,
{
    fn from(e: T) -> Self {
        error!("encountered internal server error: {}", e.to_string());
        Self
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .add(themes())
        .add(events())
        .add(daily_digest())
        .add(ical_digests())
        .add(ical_events())
        .add(event_extras())
        .add(bucket_extras())
        .add(openapi())
        .with_state(state)
}

#[endpoint(method = "GET", path = "/openapi.json", description = "OpenAPI spec")]
async fn openapi(State(_): State<AppState>) -> impl IntoResponse {
    let openapi = build_openapi(|| {
        OpenApiBuilder::new().info(InfoBuilder::new().title("Fuego").version("0.1.0"))
    });

    Json(openapi)
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
struct ICalParams {
    #[serde(rename = "theme")]
    themes: HashSet<String>,
    title: String,
    /// minimum similarity score (0 = no filtering)
    #[serde(default)]
    threshold: f64,
}

#[derive(Serialize, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct DigestParams {
    #[serde(default, rename = "theme")]
    themes: HashSet<String>,
    limit: Option<u32>,
    #[serde(default)]
    threshold: f64,
}

#[derive(Serialize, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
struct ICalDigestParams {
    #[serde(default, rename = "theme")]
    themes: HashSet<String>,
    title: String,
    /// max sources listed per digest entry; omit for all
    limit: Option<u32>,
    #[serde(default)]
    threshold: f64,
    /// how many days back to include (default 7, max 31)
    days: Option<u32>,
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
struct Records {
    #[serde(rename = "id")]
    ids: Vec<String>,
}

#[endpoint(
    method = "GET",
    path = "/themes",
    description = "A list of themes and their article counts"
)]
async fn themes(State(state): State<AppState>) -> Result<Json<Vec<Theme>>, InternalServerError> {
    // select the whole theme table since it is small enough.
    let themes: Vec<Theme> = state.db.select("theme").await?;
    Ok(Json(themes))
}

#[endpoint(
    method = "GET",
    path = "/event",
    description = "Fetch many events from their ids"
)]
async fn events(
    Query(ids): Query<Records>,
    State(state): State<AppState>,
) -> Result<Json<Vec<CalendarEventFull>>, InternalServerError> {
    // TODO bad request for invalid ids
    let mut db_res = state
        .db
        .query("fn::get_events_from_partial_ids($ids)")
        .bind(("ids", ids.ids))
        .await?;

    let events: Vec<CalendarEventFull> = db_res.take(0)?;
    Ok(Json(events))
}

#[endpoint(
    method = "GET",
    path = "/digest",
    description = "The current daily digest"
)]
async fn daily_digest(
    Query(params): Query<DigestParams>,
    State(state): State<AppState>,
) -> Result<Json<Vec<BucketWithSources>>, InternalServerError> {
    let mut db_res = state
        .db
        .query("fn::get_daily_digest($themes, time::now(), $limit, $threshold)")
        .bind(("themes", params.themes))
        .bind(("limit", params.limit))
        .bind(("threshold", params.threshold))
        .await?;

    let digest: Vec<BucketWithSources> = db_res.take(0)?;
    Ok(Json(digest))
}

const MAX_DIGEST_DAYS: u32 = 31;

#[endpoint(
    method = "GET",
    path = "/daily_digests.ics",
    description = "iCalendar stream of daily digests"
)]
async fn ical_digests(
    Query(params): Query<ICalDigestParams>,
    State(state): State<AppState>,
) -> Result<Response, InternalServerError> {
    let days = params.days.unwrap_or(7).clamp(1, MAX_DIGEST_DAYS);

    // one digest per day, stepping backwards from now
    let sql = (0..days)
        .map(|i| format!("fn::get_daily_digest($themes, time::now() - {i}d, $limit, $threshold);"))
        .collect::<Vec<_>>()
        .join("\n");

    let mut db_res = state
        .db
        .query(sql)
        .bind(("themes", params.themes))
        .bind(("limit", params.limit))
        .bind(("threshold", params.threshold))
        .await?;

    // collect buckets, deduplicating any that show up on adjacent days
    let mut seen = HashSet::new();
    let mut buckets: Vec<BucketWithSources> = Vec::new();
    for i in 0..days as usize {
        let digest: Vec<BucketWithSources> = db_res.take(i)?;
        for bucket in digest {
            if seen.insert(bucket.id.clone()) {
                buckets.push(bucket);
            }
        }
    }

    // sources are only keys, so fetch the events to list their names and links
    let source_ids: Vec<String> = buckets
        .iter()
        .flat_map(|b| b.sources.iter().cloned())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let events: Vec<CalendarEventFull> = if source_ids.is_empty() {
        Vec::new()
    } else {
        let mut res = state
            .db
            .query("fn::get_events_from_partial_ids($ids)")
            .bind(("ids", source_ids))
            .await?;
        res.take(0)?
    };
    let events_by_id: HashMap<&str, &CalendarEventFull> =
        events.iter().map(|e| (e.id.as_str(), e)).collect();

    let mut calendar = Calendar::default();
    calendar.name(&params.title);

    for bucket in &buckets {
        let links: Vec<String> = bucket
            .sources
            .iter()
            .filter_map(|id| events_by_id.get(id.as_str()))
            .map(|e| format!("- {}: {}", e.name, e.article_url))
            .collect();

        let mut description = bucket.summary.clone();
        if !links.is_empty() {
            description.push_str("\n\nSources:\n");
            description.push_str(&links.join("\n"));
        }

        calendar.push(
            Event::new()
                .summary(&format!("{} digest", bucket.theme))
                .description(&description)
                .uid(&format!("digest-{}", bucket.id))
                .all_day(bucket.end_timestamp.deref().date_naive())
                .done(),
        );
    }

    Ok(Response::builder()
        .header("Content-Type", "text/calendar")
        .body(Body::from(calendar.to_string()))?)
}

#[endpoint(
    method = "GET",
    path = "/events.ics",
    description = "iCalendar stream of individual events"
)]
async fn ical_events(
    Query(params): Query<ICalParams>,
    State(state): State<AppState>,
) -> Result<Response, InternalServerError> {
    debug!("themes = {:?}", params.themes);

    let mut db_res = state
        .db
        .query("fn::get_events_with_any_theme($themes, $threshold)")
        .bind(("themes", params.themes))
        .bind(("threshold", params.threshold))
        .await?;

    let events: Vec<CalendarEventFull> = db_res.take(0)?;

    let mut calendar = Calendar::default();
    calendar.name(&params.title);

    for event in events {
        debug!("Event: {event:?}");

        calendar.push(
            Event::new()
                .summary(&event.name)
                .description(&event.summary)
                .url(&event.article_url)
                .uid(&event.id)
                .starts(event.timestamp.deref().to_owned()),
        );
    }

    let ical = calendar.to_string();

    Ok(Response::builder()
        .header("Content-Type", "text/calendar")
        .body(Body::from(ical))?)
}

#[endpoint(
    method = "GET",
    path = "/event/extra",
    description = "Extra data (source, semantic text, embedding, metadata) for many events from their ids"
)]
async fn event_extras(
    Query(ids): Query<Records>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ExtraData>>, InternalServerError> {
    let mut db_res = state
        .db
        .query("fn::get_event_extras($ids)")
        .bind(("ids", ids.ids))
        .await?;

    let extras: Vec<ExtraData> = db_res.take(0)?;
    Ok(Json(extras))
}

#[endpoint(
    method = "GET",
    path = "/bucket/extra",
    description = "Extra data (activity, direction, description) for many digest buckets from their ids"
)]
async fn bucket_extras(
    Query(ids): Query<Records>,
    State(state): State<AppState>,
) -> Result<Json<Vec<ExtraData>>, InternalServerError> {
    let mut db_res = state
        .db
        .query("fn::get_bucket_extras($ids)")
        .bind(("ids", ids.ids))
        .await?;

    let extras: Vec<ExtraData> = db_res.take(0)?;
    Ok(Json(extras))
}
