use warp::Filter;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::convert::Infallible;

#[derive(Deserialize, Serialize)]
struct JsonRequest {
    data: String,
}

async fn handle_request(json: JsonRequest) -> Result<impl warp::Reply, Infallible> {
    let client = Client::new();
    let grpc_endpoint = std::env::var("GRPC_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:50051".to_string());
    
    let result = client
        .post(&grpc_endpoint)
        .json(&json)
        .send()
        .await;
    
    match result {
        Ok(response) => {
            match response.text().await {
                Ok(text) => Ok(warp::reply::with_status(
                    warp::reply::json(&text),
                    warp::http::StatusCode::OK,
                )),
                Err(e) => {
                    eprintln!("Error reading response text: {}", e);
                    Ok(warp::reply::with_status(
                        warp::reply::json(&format!("Error: {}", e)),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        },
        Err(e) => {
            eprintln!("Error sending request: {}", e);
            Ok(warp::reply::with_status(
                warp::reply::json(&format!("Error: {}", e)),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
}

#[tokio::main]
async fn main() {
    let json_route = warp::path("json")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_request);

    warp::serve(json_route).run(([127, 0, 0, 1], 3030)).await;
}
