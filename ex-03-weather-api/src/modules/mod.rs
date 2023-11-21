use std::fs;
use std::process;
use std::error::Error;
use serde_json::Value;

pub async fn get_weather(lat: f64, long: f64) -> Result<Value, Box<dyn Error>> {
    let api_key: String = fs::read_to_string("secret.txt").unwrap_or_else(|_err| {
        eprintln!("Error: secret.txt does not exist. Please create an OpenWeatherMap account, get your API key, and put it in a file named secret.txt in the root of this app.");
        process::exit(1);
    });
    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", lat, long, api_key);
    let response: Value = reqwest::get(url)
        .await?
        .json::<Value>() // I can also use .text() for String
        .await?;
    Ok(response)
}