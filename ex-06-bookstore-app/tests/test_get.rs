use std::net::SocketAddr;
use ex_06_bookstore_app::{
    app, 
    routes::{Book, read_books},
};

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
async fn get_books_data_recieved_matches_json() {
    let addr = activate_local_server().await;
    let books: Vec<Book> = read_books();

    let client = reqwest::Client::new();
    let response = client.get(&format!("http://{addr}/api/books")).send().await.unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let test = response.json::<Vec<Book>>().await.unwrap();
    assert_eq!(test, books);
    assert_eq!(test.len(), books.len());
}

#[tokio::test]
async fn get_book_id_receives_correct_book() {
    let addr = activate_local_server().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("http://{addr}/api/books/1"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let test = response.json::<Book>().await.unwrap();

    assert_eq!(test.id, 1);

    let books: Vec<Book> = read_books()
        .into_iter()
        .filter(|b| (*b).id == test.id)
        .collect();
    assert_eq!(&test, books.get(0).unwrap());
}