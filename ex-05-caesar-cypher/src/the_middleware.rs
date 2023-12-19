use axum::Router;
use tower::ServiceBuilder;
use http::Method;
use tracing::Level;
use tower_http::{
    trace::{ self, TraceLayer },
    cors::{ Any, CorsLayer },
    validate_request::ValidateRequestHeaderLayer,
};

pub fn add_middleware(app: Router) -> Router {
    app.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
            )
            .layer(ValidateRequestHeaderLayer::accept("application/json"))
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::POST])
                    .allow_origin(Any)
            )
    )
}