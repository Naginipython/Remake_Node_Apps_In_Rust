use std::fs;
use std::process;
use serde::Deserialize;
use rand::Rng;
use crate::quotes_struct::Quote;

#[derive(Deserialize)]
struct Temp {
    quotes: Vec<Quote>,
}

fn get_quotes() -> Vec<Quote> {
    let quotes: String = fs::read_to_string("quotes.json").unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(1);
    });
    let temp: Temp = serde_json::from_str(&quotes).expect("JSON was not well-formatted");
    temp.quotes
}

pub fn random() -> Quote {
    let quotes_vec = get_quotes();
    
    let rng = rand::thread_rng().gen_range(1..=quotes_vec.len());
    let default = Quote { quote: String::from(""), author: String::from("") };

    quotes_vec.get(rng).unwrap_or_else(|| {
        &default
    }).clone()
}

pub fn random_author(author: String) -> Vec<Quote> {
    let quotes_vec = get_quotes();

    quotes_vec.into_iter()
        .filter(|q| q.author.eq_ignore_ascii_case(&author))
        .collect()
} 

pub fn random_word(word: String) -> Vec<Quote> {
    let quotes_vec = get_quotes();

    quotes_vec.into_iter()
        .filter(|q| q.quote.to_ascii_lowercase().contains(&word.to_ascii_lowercase()))
        .collect()
}