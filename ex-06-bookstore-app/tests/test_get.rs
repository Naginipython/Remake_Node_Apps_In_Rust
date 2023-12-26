use ex_06_bookstore_app::{
    // activate_local_server, 
    routes::{Book, read_books},
};

#[tokio::test]
async fn get_books_data_recieved_matches_json() {
    // let addr = activate_local_server().await;
    let addr = "127.0.0.1:3000";
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
    // let addr = activate_local_server().await;
    let addr = "127.0.0.1:3000";

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