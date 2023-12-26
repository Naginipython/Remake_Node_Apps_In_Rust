use ex_06_bookstore_app::{
    activate_local_server,
    routes::Book,
};
use reqwest::Body;
use serde_json::{json, Value};

#[tokio::test]
async fn put_books_successfully_updates_book() {
    let addr = activate_local_server().await;

    let update = json!({
        "title": "Hunger Games",
        "author": "Suzanne Collins",
        "genre": "Survival",
        "price": 20.00,
        "quantity": 20
    });

    let id = 2;
    println!("ID: {id}");

    let client = reqwest::Client::new();
    let response = client.put(&format!("http://{addr}/api/books/{id}"))
        .body(Body::from(serde_json::to_string(&update).unwrap()))
        .header("content-type", "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let test = response.json::<Vec<Book>>().await.unwrap();
    assert!(test.iter().any(|b| b.id == id));
    let check_data = test.iter().find(|b| b.id == id).unwrap();
    assert_eq!(check_data.title, "Hunger Games");
    assert_eq!(check_data.author, "Suzanne Collins");
    assert_eq!(check_data.genre, "Survival");
    assert_eq!(check_data.price, 20.00);
    assert_eq!(check_data.quantity, 20);
}

#[tokio::test]
async fn put_books_updates_with_small_json() {
    let addr = activate_local_server().await;
    let update = json!({"title": "39 Clues"});
    let id = 1;

    let client = reqwest::Client::new();
    let response = client.put(&format!("http://{addr}/api/books/{id}"))
        .body(Body::from(serde_json::to_string(&update).unwrap()))
        .header("content-type", "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let test = response.json::<Vec<Book>>().await.unwrap();
    assert!(test.iter().any(|b| b.id == id));
    let check_data = test.iter().find(|b| b.id == id).unwrap();
    assert_eq!(check_data.title, "39 Clues");
}

#[tokio::test]
async fn put_books_fails_with_unused_id() {
    let addr = activate_local_server().await;
    let update = json!({"title": "39 Clues"});
    let unused_id = 10;

    let client = reqwest::Client::new();
    let response = client.put(&format!("http://{addr}/api/books/{unused_id}"))
        .body(Body::from(serde_json::to_string(&update).unwrap()))
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