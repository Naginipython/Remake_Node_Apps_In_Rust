use axum::{
    routing::get,
    Router,
};
pub mod routes;
pub mod quotes_struct;
pub mod view;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/quote", get(routes::quote));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Server is running on localhost:3000");
}
