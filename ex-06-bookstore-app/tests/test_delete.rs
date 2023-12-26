use ex_06_bookstore_app::{
    activate_local_server,
    routes::{Book, read_books},
};
use rand::Rng;
use serde_json::Value;

fn get_random_book_id() -> u32 {
    let books: Vec<Book> = read_books();
    // avoids the first, since test_post uses one of those
    let rng = rand::thread_rng().gen_range(2..=books.len()-1);
    println!("RNG: {rng}");
    return books.get(rng).unwrap().id;
}

fn get_random_u32() -> u32 {
    let books: Vec<Book> = read_books();
    let mut random_u32: u32;
    // Rust's stupid way of doing a `do.. while` loop
    while {
        random_u32 = rand::random::<u32>();
        books.iter().any(|b| b.id == random_u32)
    } {}
    return random_u32;
}

#[tokio::test]
async fn delete_books_works() {
    let addr = activate_local_server().await;

    let id = get_random_book_id();

    let client = reqwest::Client::new();
    let response = client.delete(&format!("http://{addr}/api/books/{id}"))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::ACCEPTED);

    let test = response.json::<Vec<Book>>().await.unwrap();
    let books: Vec<Book> = read_books();
    assert_eq!(test, books);
    assert_eq!(test.len(), books.len());
}

#[tokio::test]
async fn delete_books_fails_with_unused_id() {
    let addr = activate_local_server().await;

    let id = get_random_u32();

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