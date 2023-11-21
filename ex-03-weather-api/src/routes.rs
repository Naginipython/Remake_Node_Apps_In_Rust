use axum::{
    extract::Path,
    response::Json,
};
use serde_json::{Value, json};
use crate::modules;

pub async fn my_weather(Path((lat, long)): Path<(f64, f64)>) -> Json<Value> {
    let data: Value = modules::get_weather(lat, long).await.unwrap_or_else(|_| {
        serde_json::Value::String(String::from(""))
    });
    
    // Initially, I had data as a String, and this converted it. I changed modules/mod.rs to simply take it as json
    // Changes format to serde's JSON. 'expect' likely isn't good error handling here, but unwrap_or_else gave me issues.
    // let json: Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    
    Json(json!(data))
}