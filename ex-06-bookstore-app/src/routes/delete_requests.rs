use std::process;

use axum::{
    extract::Path, 
    response::Response, 
    body::Body
};
use http::StatusCode;
use serde_json::json;
use tracing::debug;
use super::read_books;

pub async fn remove_book(Path(id): Path<u32>) -> Response {
    let mut books = read_books();

    let index = books.iter_mut().position(|b| b.id == id);
    match index {
        Some(i) => {
            let b = books.swap_remove(i);
            debug!("Book Deleted: {b:?}");
        },
        None => {
            let json = json!({"Error": "id not in database"}).to_string();
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from(json))
                .unwrap()
        },
    }

    match super::write_books(books) {
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        },
        Ok(books) => {
            return Response::builder()
                .status(StatusCode::ACCEPTED)
                .header("content-type", "application/json")
                .body(Body::from(json!(books).to_string()))
                .unwrap();
        }
    }
}