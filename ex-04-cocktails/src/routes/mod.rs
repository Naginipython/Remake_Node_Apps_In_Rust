use axum::{
    extract,
    response::Json
};
use serde::Deserialize;
use serde_json::{ Value, json };
use crate::modules;

#[derive(Deserialize, Debug)]
pub struct DrinkExtract {
    drink: Option<String>,
}

pub struct Drink {
    drink: String,
}

pub async fn hello() -> Json<Value> {
    Json(json!({ "message": "hello world" }))
}

// ISSUE: If "drink" is NOT a String, crashes
pub async fn cocktail(extract::Json(data): extract::Json<DrinkExtract>) -> Json<Value> {
    // Note: With this, a user CAN send incorrect JSON, but it will not return anything interesting.
    let data: Drink = Drink {
        drink: data.drink.map_or(String::from(""), |v| v),
    };
    let mut result: Vec<String> = Vec::new();

    if data.drink.ne("") {
        let res: Value = modules::get_cocktails(data.drink).await.unwrap_or_default();
    
        let or_else: Vec<Value> = vec![json!([])];      // This will unwrap, or use a empty Vec<Value> (aka, loop doesn't happen)
        for i in 0..res["drinks"].as_array().unwrap_or_else(|| &or_else).len() {
            let val: String = String::from( res["drinks"][i]["strDrink"].as_str().unwrap_or_default() );
            if val.ne("") {
                result.push(val);
            }
        }
    }
    
    Json(json!(result))
}