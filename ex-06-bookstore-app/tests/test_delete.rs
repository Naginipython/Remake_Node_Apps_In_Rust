use ex_06_bookstore_app::{
    activate_local_server,
    routes::Book,
};
use serde_json::Value;

#[tokio::test]
async fn delete_books_works() {
    let addr = activate_local_server().await;

    let id = 1;

    let client = reqwest::Client::new();
    let response = client.delete(&format!("http://{addr}/api/books/{id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let test = response.json::<Vec<Book>>().await.unwrap();
    assert!(!test.iter().any(|b| b.id == id));
}

#[tokio::test]
async fn delete_books_fails_with_unused_id() {
    let addr = activate_local_server().await;

    let id = 10;

    let client = reqwest::Client::new();
    let response = client.delete(&format!("http://{addr}/api/books/{id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);
    let test = response.json::<Value>().await;
    match test {
        Ok(_data) => assert!(true),
        Err(_err) => assert!(false),
    }
}