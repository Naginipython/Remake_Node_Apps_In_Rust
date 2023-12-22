use std::net::SocketAddr;
use ex_06_bookstore_app::{
    app,
    routes::{Book, read_books},
};
use reqwest::Body;
use serde_json::{json, Value};

async fn activate_local_server() -> SocketAddr {
    // I really didn't want to call this multiple times but I'd rather it doesn't get obstructed by process ordering
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, app().await).await.unwrap();
    });
    addr
}

#[tokio::test]
async fn post_books_successfully_adds_book() {
    let addr = activate_local_server().await;
    let books: Vec<Book> = read_books();
    let mut random_u32: u32;
    // Rust's stupid way of doing a `do.. while` loop
    while {
        random_u32 = rand::random::<u32>();
        books.iter().any(|b| b.id == random_u32)
    } {}

    let json = json!({
        "id": random_u32,
        "title": "test",
        "author": "test",
        "genre": "test",
        "price": 20.00,
        "quantity": 20
    });

    let client = reqwest::Client::new();
    let response = client.post(&format!("http://{addr}/api/books"))
        .body(Body::from(serde_json::to_string(&json).unwrap()))
        .header("content-type", "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let test = response.json::<Vec<Book>>().await.unwrap();
    let books: Vec<Book> = read_books();
    assert_eq!(test, books);
    assert_eq!(test.len(), books.len());
}

#[tokio::test]
async fn post_books_doesnt_allow_same_id() {
    let addr = activate_local_server().await;
    let books: Vec<Book> = read_books();
    let used_id = books.get(0).unwrap().id;

    let json = json!({
        "id": used_id,
        "title": "test",
        "author": "test",
        "genre": "test",
        "price": 20.00,
        "quantity": 20
    });

    let client = reqwest::Client::new();
    let response = client.post(&format!("http://{addr}/api/books"))
        .body(Body::from(serde_json::to_string(&json).unwrap()))
        .header("content-type", "application/json")
        .send()
        .await
        .unwrap();

        // TODO: changed status code
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let test = response.json::<Value>().await;
    match test {
        Ok(_data) => assert!(true),
        Err(_err) => assert!(false),
    }
}