use axum::response::Html;
use axum::extract::{ Query, Path };
use serde::Deserialize;
use crate::{quotes_struct::Quote, view::view_html};

use self::random_quotes::{random_author, random_word};

pub mod random_quotes;

#[derive(Deserialize)]
pub struct PaginationParams {
    author: Option<String>,
    word: Option<String>,
}

struct QueryWords {
    author: String,
    word: String,
}

pub async fn quote(Query(pagination_params): Query<PaginationParams>) -> Html<String> {
    // I need to do this, because Query extractor uses unwrap_or_default. 
    // https://stackoverflow.com/a/75954058
    let query_words = QueryWords {
        author: pagination_params.author.map_or(String::from(""), |v| v),
        word: pagination_params.word.map_or(String::from(""), |v| v),
    };

    if !query_words.author.eq("") {
        let quotes: Vec<Quote> = random_author(query_words.author);
        view_html(quotes)
    } else if !query_words.word.eq("") {
        let quotes: Vec<Quote> = random_word(query_words.word);
        view_html(quotes)
    } else {
        let quote: Quote = random_quotes::random();
        
        view_html(vec![quote])
    }
}