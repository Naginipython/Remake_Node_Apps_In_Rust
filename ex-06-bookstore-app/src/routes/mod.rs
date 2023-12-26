use std::{
    fs::{self, /*OpenOptions,*/ File}, 
    process, 
    io::{Error, Write}
};
use serde::{Serialize, Deserialize};
use tracing::error;

pub mod get_requests;
pub mod post_requests;
pub mod put_requests;
pub mod delete_requests;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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
        error!("Error: {err}");
        process::exit(1);
    });
    
    serde_json::from_str(&books).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        error!("Error: {err}");
        process::exit(1);
    })
    // let file = File::open("books.json").expect("Error opening file");
    // let data: Vec<Book> = serde_json::from_reader(file).expect("Error getting data from file");
    // data
}

pub fn write_books(data: Vec<Book>) -> Result<Vec<Book>, Error> {
    // This caused issues with bad JSON
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(false)
    //     .open("books.json")?;
    
    // serde_json::to_writer_pretty(&mut file, &data)?;
    // Ok(data)
    // TODO: Race condition here, with tests. Need to put on a single server
    let json_data = serde_json::to_string(&data)?;
    let mut file = File::create("books.json")?;
    file.write_all(json_data.as_bytes())?;
    Ok(data)
}