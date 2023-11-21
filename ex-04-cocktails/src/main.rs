use axum::{
    routing::get,
    routing::post,
    Router,
};

pub mod routes;
pub mod modules;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/test/hello", get(routes::hello))
        .route("/test/cocktail", post(routes::cocktail));

    println!("Server is running on port localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
