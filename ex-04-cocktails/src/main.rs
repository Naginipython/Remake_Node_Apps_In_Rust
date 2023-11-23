use axum::{
    routing::get,
    routing::post,
    Router,
};
use reqwest::Method;
use tower::ServiceBuilder;
use tracing::Level;
use tower_http::{
    trace::{self, TraceLayer},
    validate_request::ValidateRequestHeaderLayer,
    cors::{ Any, CorsLayer},
};

pub mod routes;
pub mod modules;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/test/hello", get(routes::hello))
        .route("/test/cocktail", post(routes::cocktail))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                )
                .layer(ValidateRequestHeaderLayer::accept("application/json"))
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(Any)
                )
        );

    println!("Server is running on port localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
