use std::net::SocketAddr;
use axum::{
    routing::get,
    Router, Json, extract::ConnectInfo,
};
use hyper::{HeaderMap, header::{USER_AGENT, ACCEPT_LANGUAGE}};
use serde::Serialize;

#[derive(Serialize)]
struct WhoamiResponse{
    ipaddress: String,
    language: String,
    software: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/whoami", get(whoami));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn whoami(headers: HeaderMap, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Json<WhoamiResponse> {
    Json(WhoamiResponse{
        ipaddress: addr.ip().to_string(),
        software: headers[USER_AGENT].to_str().unwrap().to_string(),
        language: headers[ACCEPT_LANGUAGE].to_str().unwrap().to_string(),
    })
}