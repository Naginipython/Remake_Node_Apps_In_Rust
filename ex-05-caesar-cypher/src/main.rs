use axum::{
    routing::post,
    Router,
    middleware,
};
use crate::modules::*;

mod routes;
mod the_middleware;
pub mod modules;

async fn hello() -> &'static str {
    // exists purely to error-handle middleware
    "hello"
}
async fn hello_two() -> &'static str {
    // exists purely to error-handle middleware
    "hello, everyone in the world"
}

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let mut app = Router::new()
        .route("/testEncrypt/:shift", post(routes::encrypt))
        .route("/testDecrypt/:shift", post(routes::decrypt))
        .route("/hello", post(hello))
        .route("/helloeveryoneintheworld", post(hello_two))
        .layer(middleware::from_fn(my_middleware::my_middleware));
    
    app = the_middleware::add_middleware(app);

    // Old way, pre Axum 0.7.0
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server is running on localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
