use axum::{
    routing::get,
    Router,
};

pub mod routes;
pub mod modules;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/myweather/:lat/:long", get(routes::my_weather));

    println!("Server is running on localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
