use ex_06_bookstore_app::app;

// Moved a lot of stuff over to `lib.rs` so that I can do integrated tests
#[tokio::main]
async fn main() {
    // TODO: Info and Error file logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server is running on localhost:3000");
    axum::serve(listener, app().await).await.unwrap();
}
