use axum::{
    extract::Path,
    Json,
};
use serde_json::{Value, json};

use super::Book;

pub async fn get_books() -> Json<Vec<Book>> {
    Json(super::read_books())
}

pub async fn get_book_by_id(Path(id): Path<u32>) -> Json<Value> {
    let books: Vec<Book> = super::read_books();
    let b: Vec<Book> = books.into_iter().filter(|b| (*b).id == id).collect();

    match b.get(0) {
        Some(b) => return Json(json!(b)),
        None => return Json(json!({})),
    }
}