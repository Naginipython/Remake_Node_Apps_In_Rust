use std::error::Error;
use serde_json::Value;

pub async fn get_weather(lat: f64, long: f64) -> Result<Value, Box<dyn Error>> {
    let api_key = "6be169f62d1e7f7972ce73fd3f36cbd4";
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", lat, long, api_key);
    let response: Value = reqwest::get(url)
        .await?
        .json::<Value>() // I can also use .text() for String
        .await?;
    Ok(response)
}