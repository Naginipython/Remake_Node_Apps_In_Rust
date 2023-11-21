use std::error::Error;
use serde_json::Value;

pub async fn get_cocktails(drink: String) -> Result<Value, Box<dyn Error>> {
    // Example domain:
    // https://www.thecocktaildb.com/api/json/v1/1/search.php?s=margarita
    let url: String = format!("https://www.thecocktaildb.com/api/json/v1/1/search.php?s={}", drink);
    let response: Value = reqwest::get(url)
        .await?
        .json::<Value>()
        .await?;
    Ok(response)
}