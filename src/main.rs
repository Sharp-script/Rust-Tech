use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    axum::serve(listener_bind().await, routers_build()).await.unwrap();
    info!("Server finish init !");
}

fn routers_build() -> Router {
    let routers = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    info!("Server app router has init");

    routers
}

async fn listener_bind() -> TcpListener {
    let mut ip = String::new();
    let server_port = "6001";
    ip.push_str("127.0.0.1:");
    ip.push_str(server_port);
    let listener = TcpListener::bind(ip.clone()).await.unwrap();

    info!("Server listener has init: {}", ip);

    listener
}

async fn root() -> &'static str {
    "Hello, Rust Axum 'web application framework' !"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: rand::random(),
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}