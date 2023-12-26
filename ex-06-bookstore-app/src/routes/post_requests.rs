use std::process;
use axum::{
    extract,
    response::Response, 
    body::Body
};
use http::StatusCode;
use serde_json::json;
use tracing::debug;
use super::Book;

pub async fn post_book(extract::Json(body): extract::Json<Book>) -> Response {
    let mut books = super::read_books();

    if books.iter().any(|b| b.id == body.id) {
        let json = json!({"Error": "id already in database"}).to_string();
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(json))
            .unwrap();
    }
    // I'm figuring axum is rigid enough for this to only work if it IS a book
    // That is to say, if it can fit inside the Book struct, it can be pushed here
    debug!("Book Created: {body:?}");
    books.push(body);

    match super::write_books(books) {
        Err(err) => {
            eprint!("Error: {err}");
            process::exit(1);
        },
        Ok(books) => {
            return Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(Body::from(json!(books).to_string()))
                .unwrap();
        },
    }
}