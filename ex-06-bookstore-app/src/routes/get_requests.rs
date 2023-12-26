use axum::{
    extract::Path,
    Json,
};
use serde_json::{Value, json};

use super::{Book, BOOKS};

pub async fn get_books() -> Json<Value> {
    // Json(super::read_books())
    let books = BOOKS.lock().unwrap();
    Json(json!(*books))
}

pub async fn get_book_by_id(Path(id): Path<u32>) -> Json<Value> {
    // let books: Vec<Book> = super::read_books();
    let books = BOOKS.lock().unwrap().clone();
    let b: Vec<Book> = books.into_iter().filter(|b| (*b).id == id).collect();

    match b.get(0) {
        Some(b) => return Json(json!(b)),
        None => return Json(json!({})),
    }
}