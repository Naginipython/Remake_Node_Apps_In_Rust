// Started project with:
// cargo add axum
// cargo add tokio --features macros,rt-multi-thread
use axum::{
    // body::Body,
    routing::get,
    routing::post,
    response::Json,
    Router,
    extract::Path,
};
use serde_json::{Value, json};
// use crate::routes::*; // I use this if I don't intend on using routes::_
pub mod routes; // This exposes outside file

async fn root() -> &'static str {
    "Welcome to the kingdom of Axum!"
}

async fn about() -> &'static str {
    "Welcome to the Chronicals of Knowledge, where legends are born!"
}

async fn post_about(Path(num): Path<i32>) -> Json<Value> {
    Json(json!({ "data": num }))
}

async fn contact() -> &'static str {
    "Reach out to us, ano together, we shall conquer new frontiers!"
}

async fn post_contact() -> &'static str {
    "I can venture onwards!"
}

#[tokio::main]
async fn main() {
    // Setting single Route
    // let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = Router::new()
        .route("/", get(root))
        .route("/about", get(about))
        .route("/about/:num", post(post_about))
        .route("/contact", get(contact).post(post_contact))
        .route("/products", get(routes::get_products).post(routes::post_products));

    // Listen on port 3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Server is running on locahost:3000");
}