use axum::{
    routing::get,
    Router,
};

pub mod router;
pub mod math_router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(router::root))
        .route("/about", get(router::about))
        .route("/add/:a/:b", get(math_router::math_route));

    println!("Server is running on localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
