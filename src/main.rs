mod databases;
mod models;
mod utils;

use std::net::SocketAddr;

use axum::extract::Query;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use dotenvy::dotenv;
use models::location::{FormLocation, Location, RequestGetLocation};
use models::user::{User, UserAddFriend};
use serde::Serialize;

#[tokio::main]
async fn main() {
    match dotenv() {
        Ok(_) => (),
        Err(err) => println!("Error dotenv : {}", err.to_string()),
    };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users/add-friend", post(add_friend))
        .route("/locations", post(create_location).get(get_all_location));

    let host = std::env::var("SERVER_HOST").expect("Unable to load Server Host");
    let port = std::env::var("SERVER_PORT").expect("Unable to load Server Port");
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();

    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_location(
    Json(payload): Json<FormLocation>,
) -> (StatusCode, Json<ResponseApi<Option<Location>>>) {
    let result = match Location::create(payload).await {
        Ok(result) => result,
        Err(err) => return ResponseApi::error(StatusCode::BAD_REQUEST, err.message),
    };

    ResponseApi::new(StatusCode::CREATED, Some(result))
}

async fn get_all_location(
    Query(payload): Query<RequestGetLocation>,
) -> (StatusCode, Json<ResponseApi<Option<Vec<Location>>>>) {
    let result = match Location::get_all(&payload.username).await {
        Ok(result) => result,
        Err(err) => return ResponseApi::error(StatusCode::BAD_REQUEST, err.message),
    };

    ResponseApi::new(StatusCode::OK, Some(result))
}

async fn add_friend(Json(payload): Json<UserAddFriend>) -> (StatusCode, Json<ResponseApi<User>>) {
    let result = match User::add_friend(payload).await {
        Ok(result) => result,
        Err(err) => return ResponseApi::error(StatusCode::BAD_REQUEST, err.message),
    };

    ResponseApi::new(StatusCode::OK, result)
}

#[derive(Serialize)]
pub struct ResponseApi<T> {
    status: u16,
    data: Option<T>,
    message: Option<String>,
}

impl<T> ResponseApi<T> {
    pub fn new(status: StatusCode, data: T) -> (StatusCode, Json<ResponseApi<T>>) {
        (
            status,
            Json(Self {
                status: status.as_u16(),
                data: Some(data),
                message: None,
            }),
        )
    }

    pub fn error(status: StatusCode, message: String) -> (StatusCode, Json<ResponseApi<T>>) {
        (
            status,
            Json(Self {
                status: status.as_u16(),
                data: None,
                message: Some(message),
            }),
        )
    }
}

pub struct ModelError {
    status: i32,
    message: String,
}

pub struct ConnectionError {
    message: String,
}
