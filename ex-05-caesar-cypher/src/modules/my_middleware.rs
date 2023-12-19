use axum::{
    body::Body,
    response::{Response, IntoResponse},
    middleware::Next,
    extract::Request,
};
use http::{HeaderValue, header, StatusCode, response::Parts};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use chrono::prelude::*;
use crate::my_cypher;

pub async fn my_middleware(request: Request, next: Next) -> Response /*impl IntoResponse*/ {
    let uri = request.uri().to_string();

    // Retrieves response, splits to get json body
    let response = next.run(request).await;
    
    // In case the uri is small
    if uri.len() < 13 {
        // Return as normal
        return response.into_response();
    }
    
    // Middleware should only handle these routes
    if &uri[0..13] == "/testEncrypt/" {
        let data: Option<(Parts, i32, String)> = parse_body(uri, response).await;
        match data {
            Some((parts, shift, word)) => {
                let result = my_cypher::caesar_encrypt(word, shift);

                return new_response(result, parts).into_response();
            },
            None => {
                let err_str = json!({"Error": "Word not set or shift invalid"}).to_string();
                eprintln!("Error: word not set or shift invalid");
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("content-type", "application/json")
                    .body(Body::from(err_str))
                    .unwrap()
            },
        }
    } else if &uri[0..13] == "/testDecrypt/" {
        let data: Option<(Parts, i32, String)> = parse_body(uri, response).await;
        match data {
            Some((parts, shift, word)) => {
                let result = my_cypher::caesar_decrypt(word, shift);

                return new_response(result, parts).into_response();
            },
            None => {
                let err_str = json!({"Error": "Word not set or shift invalid"}).to_string();
                eprintln!("Error: word not set or shift invalid");
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("content-type", "application/json")
                    .body(Body::from(err_str))
                    .unwrap()
            },
        }
    } else {
        // Return as normal
        return response.into_response();
    }
}

async fn parse_body(uri: String, response: Response) -> Option<(Parts, i32, String)> {
    let (parts, body) = response.into_parts();

    // Gets json body, after getting the bytes
    let bytes = body.collect().await.unwrap().to_bytes();
    let json: Value = serde_json::from_str(std::str::from_utf8(&bytes).unwrap()).unwrap_or_default();

    // Retrieves the data needed
    let shift = (&uri[13..]).parse::<i32>().unwrap_or(-1);
    let word = json.get("word").unwrap_or(&json!("")).to_string();
    if shift < 0 || word.eq("") {
        return None;
    }
    Some((parts, shift, word))
}

fn new_response(mut result: Value, parts: Parts) -> Response {
    let local: DateTime<Local> = Local::now();
    let date = local.format("%m/%d/%Y").to_string();
    let time = local.format("%H:%M %P").to_string();

    result["date"] = json!(date);
    result["time"] = json!(time);

    let back_to_string = result.to_string();
    let len = back_to_string.len();
    let mut response = Response::from_parts(parts, Body::from(back_to_string));
    // Changes the content-length for safety (im forced to)
    response.headers_mut().insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&len.to_string()).unwrap(),
    );
    response
}