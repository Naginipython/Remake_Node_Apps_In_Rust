use std::{
    fs::{self, OpenOptions}, 
    process, 
    io::Error
};
use serde::{Serialize, Deserialize};

pub mod get_requests;
pub mod post_requests;
pub mod put_requests;
pub mod delete_requests;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub genre: String,
    pub price: f32,
    pub quantity: u32,
}

pub fn read_books() -> Vec<Book> {
    let books: String = fs::read_to_string("books.json").unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1);
    });
    let temp: Vec<Book> = serde_json::from_str(&books).expect("JSON was not well-formatted");
    temp
}

pub fn write_books(data: Vec<Book>) -> Result<Vec<Book>, Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("books.json")?;
    
    serde_json::to_writer_pretty(&mut file, &data)?;
    Ok(data)
}