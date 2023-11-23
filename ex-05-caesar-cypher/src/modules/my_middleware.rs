use axum::{
    http::Request,
    response::Response,
    middleware::Next
};

pub async fn my_middleware<B: std::fmt::Debug>(request: Request<B>, next: Next<B>) -> Response {
    // something to request
    println!("{:?}", request);
    todo!("get the Json body & shift");

    todo!("get the uri (testEncrypt OR testDecrypt (.contains?))");

    todo!("encrypt/decrypt word if shift >= 0, append time and date to Json");

    let response = next.run(request).await;
    response
}