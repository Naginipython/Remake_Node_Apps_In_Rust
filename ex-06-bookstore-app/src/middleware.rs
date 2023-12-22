use axum::Router;
use http::Method;
use tower_http::{
    trace::{self, TraceLayer}, 
    validate_request::ValidateRequestHeaderLayer, 
    cors::{CorsLayer, Any},
};
use tracing::Level;

pub fn add_middleware(app: Router) -> Router {
    app.layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
    )
    .layer(ValidateRequestHeaderLayer::accept("application/json"))
    .layer(
        CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_origin(Any)
    )
}