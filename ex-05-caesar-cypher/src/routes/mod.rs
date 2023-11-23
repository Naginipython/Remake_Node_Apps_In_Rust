use axum::{
    extract,
    response::Json
};
use serde_json::{ Value, json };
use serde::{ Serialize, Deserialize };

#[derive(Deserialize, Serialize, Debug)]
pub struct Word {
    word: String,
}

pub async fn decrypt(extract::Json(body): extract::Json<Word>) -> Json<Value> {
    Json(json!(body))
}

pub async fn encrypt(extract::Json(body): extract::Json<Word>) -> Json<Value> {
    Json(json!(body))
}