use std::process;

use axum::{
    extract::{self, Path}, 
    response::Response, 
    body::Body
};
use http::StatusCode;
use serde_json::json;
use serde::Deserialize;
use tracing::debug;
use super::Book;

#[derive(Deserialize, Debug)]
pub struct BookData {
    title: Option<String>,
    author: Option<String>,
    genre: Option<String>,
    price: Option<f32>,
    quantity: Option<u32>,
}

pub async fn update_book(
    Path(id): Path<u32>,
    extract::Json(body): extract::Json<BookData>,
) -> Response {
    let mut books: Vec<Book> = super::read_books();
    
    // Checking if ID exists, to update
    let book = books.iter_mut().find(|b| b.id == id);
    match book {
        Some(b) => {
            // update book in question, as long as the body contains data to update
            if let Some(t) = body.title {
                b.title = t;
            }
            if let Some(a) = body.author {
                b.author = a;
            }
            if let Some(g) = body.genre {
                b.genre = g;
            }
            if let Some(p) = body.price {
                b.price = p;
            }
            if let Some(q) = body.quantity {
                b.quantity = q;
            }
            debug!("Book Updated: {b:?}");
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