use std::process;

use axum::{extract, Json};
use serde_json::{json, Value};
use super::Book;

pub async fn post_book(extract::Json(body): extract::Json<Book>) -> Json<Value> {
    let mut books = super::read_books();

    if books.iter().any(|b| b.id == body.id) {
        // TODO: response creation, status 400
        return Json(json!({"Error": "id already in database"}));
    }
    // I'm figuring axum is rigid enough for this to only work if it IS a book
    // That is to say, if it can fit inside the Book struct, it can be pushed here
    books.push(body);

    match super::write_books(books) {
        Err(err) => {
            eprint!("Error: {err}");
            process::exit(1);
        },
        // TODO: response creation, status [new item created]
        Ok(books) => return Json(json!(books)),
    }
}