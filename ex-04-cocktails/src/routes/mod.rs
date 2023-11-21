use axum::{
    extract,
    response::Json
};
use serde::Deserialize;
use serde_json::{ Value, json };
use crate::modules;

#[derive(Deserialize, Debug)]
pub struct Drink {
    drink: String,
}

pub async fn hello() -> Json<Value> {
    Json(json!({ "message": "hello world" }))
}

pub async fn cocktail(extract::Json(data): extract::Json<Drink>) -> Json<Value> {
    // I get a valid response OR an empty String
    let res: Value = modules::get_cocktails(data.drink).await.unwrap_or_else(|_| {
        serde_json::Value::String(String::from(""))
    });
    Json(json!(res))
}