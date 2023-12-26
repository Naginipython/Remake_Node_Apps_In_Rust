use std::net::SocketAddr;
use axum::{
    routing::{get, post, put, delete},
    Router
};
use routes::{
    get_requests,
    post_requests,
    put_requests,
    delete_requests,
};

pub mod middleware;
pub mod routes;

// To be used by `main.rs` and `tests`
pub async fn app() -> Router {
    let mut app = Router::new()
        .route("/api/books", get(get_requests::get_books))
        .route("/api/books/:id", get(get_requests::get_book_by_id))
        .route("/api/books", post(post_requests::post_book))
        .route("/api/books/:id", put(put_requests::update_book))
        .route("/api/books/:id", delete(delete_requests::remove_book));

    // adds middleware
    app = middleware::add_middleware(app);
    app
}

// This is for integrated tests
pub async fn activate_local_server() -> SocketAddr {
    // I really didn't want to call this multiple times but I'd rather it doesn't get obstructed by process ordering
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app().await).await.unwrap();
    });
    addr
}