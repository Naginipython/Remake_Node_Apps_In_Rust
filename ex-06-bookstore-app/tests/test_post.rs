use ex_06_bookstore_app::{
    activate_local_server,
    routes::Book,
};
use reqwest::Body;
use serde_json::{json, Value};

#[tokio::test]
async fn post_books_successfully_adds_book() {
    let addr = activate_local_server().await;
    let id = 3;

    let json = json!({
        "id": id,
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

    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let test = response.json::<Vec<Book>>().await.unwrap();
    assert!(test.iter().any(|b| b.id == id));
    let check_data = test.iter().find(|b| b.id == id).unwrap();
    assert_eq!(check_data.title, "test");
    assert_eq!(check_data.author, "test");
    assert_eq!(check_data.genre, "test");
    assert_eq!(check_data.price, 20.00);
    assert_eq!(check_data.quantity, 20);
}

#[tokio::test]
async fn post_books_doesnt_allow_same_id() {
    let addr = activate_local_server().await;
    let used_id = 1;

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

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

    let test = response.json::<Value>().await;
    match test {
        Ok(_data) => assert!(true),
        Err(_err) => assert!(false),
    }
}

#[tokio::test]
async fn post_books_doesnt_allow_bad_data() {
    let addr = activate_local_server().await;

    let bad_json = json!({
        "id": 10,
        "title": "test",
        "author": "test",
        "genre": 5, // requires String
        "price": 20.00,
        "quantity": 20
    });

    let client = reqwest::Client::new();
    let response = client.post(&format!("http://{addr}/api/books"))
        .body(Body::from(serde_json::to_string(&bad_json).unwrap()))
        .header("content-type", "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::UNPROCESSABLE_ENTITY);
}