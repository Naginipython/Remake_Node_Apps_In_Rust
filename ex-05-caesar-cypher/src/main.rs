use axum::{
    routing::post,
    Router,
    middleware,
};
use tower::ServiceBuilder;
use reqwest::Method;
use tracing::Level;
use tower_http::{
    trace::{ self, TraceLayer },
    cors::{ Any, CorsLayer },
    validate_request::ValidateRequestHeaderLayer,
};
use crate::modules::*;

pub mod routes;
pub mod modules;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = Router::new()
        .route("/testEncrypt/:shift", post(routes::encrypt))
        .route("/testDecrypt/:shift", post(routes::decrypt))
        .layer(middleware::from_fn(my_middleware::my_middleware))
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
                        .allow_methods([Method::POST])
                        .allow_origin(Any)
                )
        );

    println!("Server is running on localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
