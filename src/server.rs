use std::collections::HashSet;
use std::ops::Deref;

use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, IntoResponseParts, ResponseParts};
use axum::{Json, Router};
use axum::response::Response;
use axum_extra::extract::Query;
use axum_openapi3::utoipa::openapi::{InfoBuilder, OpenApiBuilder, Type};
use icalendar::{Calendar, Event, Component, EventLike};
use serde::{Deserialize, Serialize};
use axum_openapi3::{AddRoute, build_openapi, endpoint, utoipa};
use axum_openapi3::utoipa::*;
use surrealdb::types::{RecordId, RecordIdKey};
use tracing::{debug, error};

use crate::AppState;
use crate::db::{CalendarEventFull, Theme};

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
        .add(ical_summary())
        .add(ical_events())
        .add(openapi())
        .with_state(state)
}

#[endpoint(method = "GET", path = "/openapi.json", description = "OpenAPI spec")]
async fn openapi(
    State(_): State<AppState>,
) -> impl IntoResponse {
    let openapi = build_openapi(|| {
        OpenApiBuilder::new().info(InfoBuilder::new().title("Fuego").version("0.1.0"))
    });

    Json(openapi)
}

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
struct ICalParams {
    themes: HashSet<String>,
    title: String,
}

#[endpoint(method = "GET", path = "/themes", description = "A list of themes and their article counts")]
async fn themes(
    State(state): State<AppState>,
) -> Result<Json<Vec<Theme>>, InternalServerError> {
    // select the whole theme table since it is small enough.
    let themes: Vec<Theme> = state.db.select("theme").await?;
    Ok(Json(themes))
}

#[endpoint(method = "GET", path = "/daily_digest.ics", description = "iCalender stream of daily digests")]
async fn ical_summary(
    Query(params): Query<ICalParams>,
    State(state): State<AppState>,
) -> Response {
    todo!()
}

fn record_to_string(record: RecordId) -> Result<String, InternalServerError> {
    match record.key {
        RecordIdKey::String(s) => Ok(s),
        _ => Err(InternalServerError::from("record key was not a string"))
    }
}

#[endpoint(method = "GET", path = "/events.ics", description = "iCalendar stream of individual events")]
async fn ical_events(
    Query(params): Query<ICalParams>,
    State(state): State<AppState>,
) -> Result<Response, InternalServerError> {
    debug!("themes = {:?}", params.themes);

    let mut db_res = state.db
        .query("fn::get_events_with_any_theme($themes)")
        .bind(("themes", params.themes))
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
                .uid(&record_to_string(event.id)?)
                .starts(event.timestamp.deref().to_owned())
        );
    }

    let ical = calendar.to_string();

    Ok(Response::builder()
        .header("Content-Type", "text/calendar")
        .body(Body::from(ical))?
    )
}