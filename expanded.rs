#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use surrealdb::{
    Surreal, engine::remote::ws::{Client, Wss},
    opt::auth::{Database, Root},
};
use tracing::info;
use tracing_subscriber::EnvFilter;
mod server {
    use std::collections::HashSet;
    use axum::body::Body;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, IntoResponseParts, ResponseParts};
    use axum::{Json, Router};
    use axum::{extract::Query, response::Response};
    use axum_openapi3::utoipa::openapi::{
        InfoBuilder, KnownFormat, OpenApiBuilder, SchemaFormat, Type,
    };
    use serde::{Deserialize, Serialize};
    use axum_openapi3::{AddRoute, build_openapi, endpoint, utoipa};
    use axum_openapi3::utoipa::*;
    use tracing::error;
    use crate::AppState;
    use crate::db::Theme;
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
            {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "event src/server.rs:34",
                            "steelhacks_xiii::server",
                            ::tracing::Level::ERROR,
                            ::tracing_core::__macro_support::Option::Some(
                                "src/server.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(34u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "steelhacks_xiii::server",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["message"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::EVENT,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let enabled = ::tracing::Level::ERROR
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::ERROR
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        let interest = __CALLSITE.interest();
                        !interest.is_never()
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                    };
                if enabled {
                    (|value_set: ::tracing::field::ValueSet| {
                        let meta = __CALLSITE.metadata();
                        ::tracing::Event::dispatch(meta, &value_set);
                        if match ::tracing::Level::ERROR {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::ERROR {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let meta = __CALLSITE.metadata();
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target(meta.target())
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            ::tracing::__macro_support::__tracing_log(
                                                meta,
                                                logger,
                                                log_meta,
                                                &value_set,
                                            )
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                    })({
                        #[allow(unused_imports)]
                        use ::tracing::field::{debug, display, Value};
                        __CALLSITE
                            .metadata()
                            .fields()
                            .value_set_all(
                                &[
                                    (::tracing::__macro_support::Option::Some(
                                        &format_args!(
                                            "encountered internal server error: {0}",
                                            e.to_string(),
                                        ) as &dyn ::tracing::field::Value,
                                    )),
                                ],
                            )
                    });
                } else {
                    if match ::tracing::Level::ERROR {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::ERROR {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &{
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set_all(
                                                        &[
                                                            (::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "encountered internal server error: {0}",
                                                                    e.to_string(),
                                                                ) as &dyn ::tracing::field::Value,
                                                            )),
                                                        ],
                                                    )
                                            },
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                }
            };
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
    fn openapi() -> (
        &'static str,
        axum::routing::MethodRouter<AppState, std::convert::Infallible>,
    ) {
        async fn openapi(State(_): State<AppState>) -> impl IntoResponse {
            let openapi = build_openapi(|| {
                OpenApiBuilder::new()
                    .info(InfoBuilder::new().title("Fuego").version("0.1.0"))
            });
            Json(openapi)
        }
        let handler = axum::routing::get(openapi);
        let op_builder = axum_openapi3::utoipa::openapi::path::OperationBuilder::new()
            .description(Some("OpenAPI spec"));
        let op_builder = op_builder;
        let op_builder = op_builder;
        let op_builder = op_builder;
        let op_builder = op_builder;
        let op_builder = op_builder.operation_id(Some("openapi"));
        let paths = axum_openapi3::utoipa::openapi::PathsBuilder::new()
            .path(
                "/openapi.json",
                axum_openapi3::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        axum_openapi3::utoipa::openapi::HttpMethod::Get,
                        op_builder.build(),
                    )
                    .build(),
            )
            .build();
        axum_openapi3::ENDPOINTS.lock().unwrap().push(paths);
        ("/openapi.json", handler)
    }
    #[into_params(parameter_in = Query)]
    struct Filters {
        themes: HashSet<String>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Filters {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private229::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Filters",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "themes",
                    &self.themes,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Filters {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private229::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private229::Formatter,
                    ) -> _serde::__private229::fmt::Result {
                        _serde::__private229::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private229::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private229::Ok(__Field::__field0),
                            _ => _serde::__private229::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private229::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "themes" => _serde::__private229::Ok(__Field::__field0),
                            _ => _serde::__private229::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private229::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"themes" => _serde::__private229::Ok(__Field::__field0),
                            _ => _serde::__private229::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private229::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private229::PhantomData<Filters>,
                    lifetime: _serde::__private229::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Filters;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private229::Formatter,
                    ) -> _serde::__private229::fmt::Result {
                        _serde::__private229::Formatter::write_str(
                            __formatter,
                            "struct Filters",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private229::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            HashSet<String>,
                        >(&mut __seq)? {
                            _serde::__private229::Some(__value) => __value,
                            _serde::__private229::None => {
                                return _serde::__private229::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Filters with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private229::Ok(Filters { themes: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private229::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private229::Option<
                            HashSet<String>,
                        > = _serde::__private229::None;
                        while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private229::Option::is_some(&__field0) {
                                        return _serde::__private229::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("themes"),
                                        );
                                    }
                                    __field0 = _serde::__private229::Some(
                                        _serde::de::MapAccess::next_value::<
                                            HashSet<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private229::Some(__field0) => __field0,
                            _serde::__private229::None => {
                                _serde::__private229::de::missing_field("themes")?
                            }
                        };
                        _serde::__private229::Ok(Filters { themes: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["themes"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Filters",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private229::PhantomData::<Filters>,
                        lifetime: _serde::__private229::PhantomData,
                    },
                )
            }
        }
    };
    impl utoipa::IntoParams for Filters {
        fn into_params(
            parameter_in_provider: impl Fn(
            ) -> Option<utoipa::openapi::path::ParameterIn>,
        ) -> Vec<utoipa::openapi::path::Parameter> {
            [
                Some(
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name("themes")
                        .parameter_in(utoipa::openapi::path::ParameterIn::Query)
                        .required(utoipa::openapi::Required::True)
                        .schema(
                            Some(
                                utoipa::openapi::schema::ArrayBuilder::new()
                                    .items(
                                        utoipa::openapi::ObjectBuilder::new()
                                            .schema_type(
                                                utoipa::openapi::schema::SchemaType::new(
                                                    utoipa::openapi::schema::Type::String,
                                                ),
                                            ),
                                    )
                                    .unique_items(true),
                            ),
                        )
                        .build(),
                ),
            ]
                .into_iter()
                .filter(Option::is_some)
                .flatten()
                .collect()
        }
    }
    impl utoipa::__dev::ComposeSchema for Filters {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "themes",
                        utoipa::openapi::schema::ArrayBuilder::new()
                            .items(
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(
                                        utoipa::openapi::schema::SchemaType::new(
                                            utoipa::openapi::schema::Type::String,
                                        ),
                                    ),
                            )
                            .unique_items(true),
                    )
                    .required("themes");
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for Filters {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("Filters")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    fn themes() -> (
        &'static str,
        axum::routing::MethodRouter<AppState, std::convert::Infallible>,
    ) {
        async fn themes(
            State(state): State<AppState>,
        ) -> Result<Json<Vec<Theme>>, InternalServerError> {
            let themes: Vec<Theme> = state.db.select("theme").await?;
            Ok(Json(themes))
        }
        let handler = axum::routing::get(themes);
        let op_builder = axum_openapi3::utoipa::openapi::path::OperationBuilder::new()
            .description(Some("A list of themes and their article counts"));
        let op_builder = op_builder;
        let op_builder = op_builder;
        let op_builder = op_builder;
        let op_builder = op_builder;
        let op_builder = op_builder.operation_id(Some("themes"));
        let paths = axum_openapi3::utoipa::openapi::PathsBuilder::new()
            .path(
                "/themes",
                axum_openapi3::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        axum_openapi3::utoipa::openapi::HttpMethod::Get,
                        op_builder.build(),
                    )
                    .build(),
            )
            .build();
        axum_openapi3::ENDPOINTS.lock().unwrap().push(paths);
        ("/themes", handler)
    }
    fn ical_summary() -> (
        &'static str,
        axum::routing::MethodRouter<AppState, std::convert::Infallible>,
    ) {
        async fn ical_summary(
            Query(filters): Query<Filters>,
            State(state): State<AppState>,
        ) -> Response {
            ::core::panicking::panic("not yet implemented")
        }
        let handler = axum::routing::get(ical_summary);
        let op_builder = axum_openapi3::utoipa::openapi::path::OperationBuilder::new()
            .description(Some("iCalender stream of daily digests"));
        let op_builder = op_builder;
        let op_builder = op_builder;
        let query_params = <Filters as axum_openapi3::utoipa::IntoParams>::into_params(||
        Some(axum_openapi3::utoipa::openapi::path::ParameterIn::Query));
        let op_builder = op_builder.parameters(Some(query_params));
        let op_builder = op_builder;
        let op_builder = op_builder.operation_id(Some("ical_summary"));
        let paths = axum_openapi3::utoipa::openapi::PathsBuilder::new()
            .path(
                "/daily_digest.ics",
                axum_openapi3::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        axum_openapi3::utoipa::openapi::HttpMethod::Get,
                        op_builder.build(),
                    )
                    .build(),
            )
            .build();
        axum_openapi3::ENDPOINTS.lock().unwrap().push(paths);
        ("/daily_digest.ics", handler)
    }
    fn ical_events() -> (
        &'static str,
        axum::routing::MethodRouter<AppState, std::convert::Infallible>,
    ) {
        async fn ical_events(
            Query(filters): Query<Filters>,
            State(state): State<AppState>,
        ) -> Response {
            ::core::panicking::panic("not yet implemented")
        }
        let handler = axum::routing::get(ical_events);
        let op_builder = axum_openapi3::utoipa::openapi::path::OperationBuilder::new()
            .description(Some("iCalendar stream of individual events"));
        let op_builder = op_builder;
        let op_builder = op_builder;
        let query_params = <Filters as axum_openapi3::utoipa::IntoParams>::into_params(||
        Some(axum_openapi3::utoipa::openapi::path::ParameterIn::Query));
        let op_builder = op_builder.parameters(Some(query_params));
        let op_builder = op_builder;
        let op_builder = op_builder.operation_id(Some("ical_events"));
        let paths = axum_openapi3::utoipa::openapi::PathsBuilder::new()
            .path(
                "/events.ics",
                axum_openapi3::utoipa::openapi::path::PathItemBuilder::new()
                    .operation(
                        axum_openapi3::utoipa::openapi::HttpMethod::Get,
                        op_builder.build(),
                    )
                    .build(),
            )
            .build();
        axum_openapi3::ENDPOINTS.lock().unwrap().push(paths);
        ("/events.ics", handler)
    }
}
mod db {
    use std::collections::HashSet;
    use surrealdb::types::{Datetime, SurrealValue, Uuid};
    pub struct CalendarEventFull {
        pub id: Uuid,
        pub summary: String,
        pub themes: HashSet<String>,
        pub article_url: String,
        pub timestamp: Datetime,
    }
    impl SurrealValue for CalendarEventFull {
        fn into_value(self) -> ::surrealdb::types::Value {
            let Self { id, summary, themes, article_url, timestamp } = self;
            {
                let mut map = ::surrealdb::types::Object::new();
                map.insert("id".to_string(), id.into_value());
                map.insert("summary".to_string(), summary.into_value());
                map.insert("themes".to_string(), themes.into_value());
                map.insert("article_url".to_string(), article_url.into_value());
                map.insert("timestamp".to_string(), timestamp.into_value());
                ::surrealdb::types::Value::Object(map)
            }
        }
        fn from_value(
            value: ::surrealdb::types::Value,
        ) -> std::result::Result<Self, ::surrealdb::types::Error> {
            if let ::surrealdb::types::Value::Object(mut map) = value {
                {
                    let field_value = map.remove("id").unwrap_or_default();
                    let id = <Uuid as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "id",
                                        "CalendarEventFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("summary").unwrap_or_default();
                    let summary = <String as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "summary",
                                        "CalendarEventFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("themes").unwrap_or_default();
                    let themes = <HashSet<
                        String,
                    > as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "themes",
                                        "CalendarEventFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("article_url").unwrap_or_default();
                    let article_url = <String as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "article_url",
                                        "CalendarEventFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("timestamp").unwrap_or_default();
                    let timestamp = <Datetime as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "timestamp",
                                        "CalendarEventFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    Ok(Self {
                        id,
                        summary,
                        themes,
                        article_url,
                        timestamp,
                    })
                }
            } else {
                let err = ::surrealdb::types::ConversionError::from_value(
                    ::surrealdb::types::Kind::Object,
                    &value,
                );
                Err(err.into())
            }
        }
        fn is_value(value: &::surrealdb::types::Value) -> bool {
            if let ::surrealdb::types::Value::Object(map) = value {
                {
                    let mut valid = true;
                    if valid {
                        if let Some(v) = map.get("id") {
                            if !<Uuid as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("summary") {
                            if !<String as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("themes") {
                            if !<HashSet<String> as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("article_url") {
                            if !<String as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("timestamp") {
                            if !<Datetime as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        return true;
                    }
                }
            }
            false;
            false
        }
        fn kind_of() -> ::surrealdb::types::Kind {
            {
                let mut map = std::collections::BTreeMap::new();
                map.insert("id".to_string(), <Uuid as SurrealValue>::kind_of());
                map.insert("summary".to_string(), <String as SurrealValue>::kind_of());
                map.insert(
                    "themes".to_string(),
                    <HashSet<String> as SurrealValue>::kind_of(),
                );
                map.insert(
                    "article_url".to_string(),
                    <String as SurrealValue>::kind_of(),
                );
                map.insert(
                    "timestamp".to_string(),
                    <Datetime as SurrealValue>::kind_of(),
                );
                ::surrealdb::types::Kind::Literal(
                    ::surrealdb::types::KindLiteral::Object(map),
                )
            }
        }
    }
    pub struct SummaryBucketFull {
        pub id: Uuid,
        pub summary: String,
        pub begin_timestamp: Datetime,
        pub end_timestamp: Datetime,
    }
    impl SurrealValue for SummaryBucketFull {
        fn into_value(self) -> ::surrealdb::types::Value {
            let Self { id, summary, begin_timestamp, end_timestamp } = self;
            {
                let mut map = ::surrealdb::types::Object::new();
                map.insert("id".to_string(), id.into_value());
                map.insert("summary".to_string(), summary.into_value());
                map.insert("begin_timestamp".to_string(), begin_timestamp.into_value());
                map.insert("end_timestamp".to_string(), end_timestamp.into_value());
                ::surrealdb::types::Value::Object(map)
            }
        }
        fn from_value(
            value: ::surrealdb::types::Value,
        ) -> std::result::Result<Self, ::surrealdb::types::Error> {
            if let ::surrealdb::types::Value::Object(mut map) = value {
                {
                    let field_value = map.remove("id").unwrap_or_default();
                    let id = <Uuid as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "id",
                                        "SummaryBucketFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("summary").unwrap_or_default();
                    let summary = <String as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "summary",
                                        "SummaryBucketFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("begin_timestamp").unwrap_or_default();
                    let begin_timestamp = <Datetime as SurrealValue>::from_value(
                            field_value,
                        )
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "begin_timestamp",
                                        "SummaryBucketFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("end_timestamp").unwrap_or_default();
                    let end_timestamp = <Datetime as SurrealValue>::from_value(
                            field_value,
                        )
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "end_timestamp",
                                        "SummaryBucketFull",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    Ok(Self {
                        id,
                        summary,
                        begin_timestamp,
                        end_timestamp,
                    })
                }
            } else {
                let err = ::surrealdb::types::ConversionError::from_value(
                    ::surrealdb::types::Kind::Object,
                    &value,
                );
                Err(err.into())
            }
        }
        fn is_value(value: &::surrealdb::types::Value) -> bool {
            if let ::surrealdb::types::Value::Object(map) = value {
                {
                    let mut valid = true;
                    if valid {
                        if let Some(v) = map.get("id") {
                            if !<Uuid as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("summary") {
                            if !<String as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("begin_timestamp") {
                            if !<Datetime as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("end_timestamp") {
                            if !<Datetime as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        return true;
                    }
                }
            }
            false;
            false
        }
        fn kind_of() -> ::surrealdb::types::Kind {
            {
                let mut map = std::collections::BTreeMap::new();
                map.insert("id".to_string(), <Uuid as SurrealValue>::kind_of());
                map.insert("summary".to_string(), <String as SurrealValue>::kind_of());
                map.insert(
                    "begin_timestamp".to_string(),
                    <Datetime as SurrealValue>::kind_of(),
                );
                map.insert(
                    "end_timestamp".to_string(),
                    <Datetime as SurrealValue>::kind_of(),
                );
                ::surrealdb::types::Kind::Literal(
                    ::surrealdb::types::KindLiteral::Object(map),
                )
            }
        }
    }
    pub struct Theme {
        pub name: String,
        pub event_count: u32,
    }
    impl SurrealValue for Theme {
        fn into_value(self) -> ::surrealdb::types::Value {
            let Self { name, event_count } = self;
            {
                let mut map = ::surrealdb::types::Object::new();
                map.insert("name".to_string(), name.into_value());
                map.insert("event_count".to_string(), event_count.into_value());
                ::surrealdb::types::Value::Object(map)
            }
        }
        fn from_value(
            value: ::surrealdb::types::Value,
        ) -> std::result::Result<Self, ::surrealdb::types::Error> {
            if let ::surrealdb::types::Value::Object(mut map) = value {
                {
                    let field_value = map.remove("name").unwrap_or_default();
                    let name = <String as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "name",
                                        "Theme",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    let field_value = map.remove("event_count").unwrap_or_default();
                    let event_count = <u32 as SurrealValue>::from_value(field_value)
                        .map_err(|e| ::surrealdb::types::Error::internal(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize field \'{0}\' on type \'{1}\': {2}",
                                        "event_count",
                                        "Theme",
                                        e,
                                    ),
                                )
                            }),
                        ))?;
                    Ok(Self { name, event_count })
                }
            } else {
                let err = ::surrealdb::types::ConversionError::from_value(
                    ::surrealdb::types::Kind::Object,
                    &value,
                );
                Err(err.into())
            }
        }
        fn is_value(value: &::surrealdb::types::Value) -> bool {
            if let ::surrealdb::types::Value::Object(map) = value {
                {
                    let mut valid = true;
                    if valid {
                        if let Some(v) = map.get("name") {
                            if !<String as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        if let Some(v) = map.get("event_count") {
                            if !<u32 as SurrealValue>::is_value(v) {
                                valid = false;
                            }
                        } else {
                            valid = false;
                        }
                    }
                    if valid {
                        return true;
                    }
                }
            }
            false;
            false
        }
        fn kind_of() -> ::surrealdb::types::Kind {
            {
                let mut map = std::collections::BTreeMap::new();
                map.insert("name".to_string(), <String as SurrealValue>::kind_of());
                map.insert("event_count".to_string(), <u32 as SurrealValue>::kind_of());
                ::surrealdb::types::Kind::Literal(
                    ::surrealdb::types::KindLiteral::Object(map),
                )
            }
        }
    }
}
struct AppState {
    db: Surreal<Client>,
}
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState {
            db: ::core::clone::Clone::clone(&self.db),
        }
    }
}
fn get_env(name: &str) -> String {
    std::env::var(name)
        .expect(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("Missing environment variable: {0}", name),
                )
            }),
        )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        dotenvy::dotenv().ok();
        tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| EnvFilter::from("INFO")),
            )
            .init();
        let surreal_uri = get_env("SURREAL_URI");
        let surreal_user = get_env("SURREAL_USER");
        let surreal_pass = get_env("SURREAL_PASS");
        let addr = get_env("ADDR");
        let db = Surreal::new::<Wss>(surreal_uri).await?;
        db.signin(Database {
                namespace: "main".into(),
                database: "main".into(),
                username: surreal_user,
                password: surreal_pass,
            })
            .await?;
        db.use_ns("main").use_db("main").await?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:44",
                        "steelhacks_xiii",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(44u32),
                        ::tracing_core::__macro_support::Option::Some("steelhacks_xiii"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set_all(
                            &[
                                (::tracing::__macro_support::Option::Some(
                                    &format_args!("Connected to SurrealDB successfully")
                                        as &dyn ::tracing::field::Value,
                                )),
                            ],
                        )
                });
            } else {
                if match ::tracing::Level::INFO {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                } <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set_all(
                                                    &[
                                                        (::tracing::__macro_support::Option::Some(
                                                            &format_args!("Connected to SurrealDB successfully")
                                                                as &dyn ::tracing::field::Value,
                                                        )),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        let app = server::router(AppState { db });
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        {
            use ::tracing::__macro_support::Callsite as _;
            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "event src/main.rs:51",
                        "steelhacks_xiii",
                        ::tracing::Level::INFO,
                        ::tracing_core::__macro_support::Option::Some("src/main.rs"),
                        ::tracing_core::__macro_support::Option::Some(51u32),
                        ::tracing_core::__macro_support::Option::Some("steelhacks_xiii"),
                        ::tracing_core::field::FieldSet::new(
                            &["message"],
                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                        ),
                        ::tracing::metadata::Kind::EVENT,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let enabled = ::tracing::Level::INFO
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::INFO
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    let interest = __CALLSITE.interest();
                    !interest.is_never()
                        && ::tracing::__macro_support::__is_enabled(
                            __CALLSITE.metadata(),
                            interest,
                        )
                };
            if enabled {
                (|value_set: ::tracing::field::ValueSet| {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Event::dispatch(meta, &value_set);
                    if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                use ::tracing::log;
                                let level = match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                };
                                if level <= log::max_level() {
                                    let meta = __CALLSITE.metadata();
                                    let log_meta = log::Metadata::builder()
                                        .level(level)
                                        .target(meta.target())
                                        .build();
                                    let logger = log::logger();
                                    if logger.enabled(&log_meta) {
                                        ::tracing::__macro_support::__tracing_log(
                                            meta,
                                            logger,
                                            log_meta,
                                            &value_set,
                                        )
                                    }
                                }
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                })({
                    #[allow(unused_imports)]
                    use ::tracing::field::{debug, display, Value};
                    __CALLSITE
                        .metadata()
                        .fields()
                        .value_set_all(
                            &[
                                (::tracing::__macro_support::Option::Some(
                                    &format_args!("Listening on {0}", addr)
                                        as &dyn ::tracing::field::Value,
                                )),
                            ],
                        )
                });
            } else {
                if match ::tracing::Level::INFO {
                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                    _ => ::tracing::log::Level::Trace,
                } <= ::tracing::log::STATIC_MAX_LEVEL
                {
                    if !::tracing::dispatcher::has_been_set() {
                        {
                            use ::tracing::log;
                            let level = match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            };
                            if level <= log::max_level() {
                                let meta = __CALLSITE.metadata();
                                let log_meta = log::Metadata::builder()
                                    .level(level)
                                    .target(meta.target())
                                    .build();
                                let logger = log::logger();
                                if logger.enabled(&log_meta) {
                                    ::tracing::__macro_support::__tracing_log(
                                        meta,
                                        logger,
                                        log_meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set_all(
                                                    &[
                                                        (::tracing::__macro_support::Option::Some(
                                                            &format_args!("Listening on {0}", addr)
                                                                as &dyn ::tracing::field::Value,
                                                        )),
                                                    ],
                                                )
                                        },
                                    )
                                }
                            }
                        }
                    } else {
                        {}
                    }
                } else {
                    {}
                };
            }
        };
        axum::serve(listener, app).await?;
        Ok(())
    };
    let body = {
        if false {
            let _: &dyn ::core::future::Future<
                Output = Result<(), Box<dyn std::error::Error>>,
            > = &body;
        }
        body
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
