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
    let response = client
        .post("http://localhost:50051")
        .json(&json)
        .send()
        .await
        .unwrap();

    Ok(warp::reply::json(&response.text().await.unwrap()))
}

#[tokio::main]
async fn main() {
    let json_route = warp::path("json")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_request);

    warp::serve(json_route).run(([127, 0, 0, 1], 3030)).await;
}
