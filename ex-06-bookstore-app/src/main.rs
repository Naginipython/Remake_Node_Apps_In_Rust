use ex_06_bookstore_app::app;
use std::fs::OpenOptions;
use tracing::{info, error, Level, debug};
use tracing_subscriber::{
    prelude::*,
    fmt,
    layer::Layer,
    Registry, filter
};

// Moved a lot of stuff over to `lib.rs` so that I can do integrated tests
#[tokio::main]
async fn main() {
    // Old, simple way I did it prior
    // tracing_subscriber::fmt()
    //     .with_target(false)
    //     .compact()
    //     .init();

    let err_file = OpenOptions::new()
        .append(true)
        .create(true) // Create if it doesn't exist
        .open("log-error.log")
        .unwrap();
    let debug_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("log-debug.log")
        .unwrap();

    let subscriber = Registry::default()
        .with(
            // stdout layer, to view everything in the console
            fmt::layer()
                .compact()
                .with_ansi(true)
        )
        .with(
            // log-error file, to log the errors that arise
            fmt::layer()
                .json()
                .with_writer(err_file)
                .with_filter(filter::LevelFilter::from_level(Level::ERROR))
        )
        .with(
            fmt::layer()
                .json()
                .with_writer(debug_file)
                .with_filter(filter::LevelFilter::from_level(Level::DEBUG))
        );
    
    tracing::subscriber::set_global_default(subscriber).unwrap();
    
    // Opens the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server is running on localhost:3000");
    error!("test");
    info!("test2");
    debug!("test3");
    axum::serve(listener, app().await).await.unwrap();
}
