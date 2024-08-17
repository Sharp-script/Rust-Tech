use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
    extract::Path
};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
        .route("/users", post(create_user))
        .route("/start_clean", get(start_clean))
        .route("/finish_clean/:order_no", get(finish_clean))
        ;

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

async fn start_clean() -> (StatusCode, Json<String>) {
    let client = reqwest::Client::new();
    let req = json!({
        "memberId" : 1411424612450304i64,
        "deviceId" : "202304060300309",
        "amount"   : 50f32
    });
    let response = client
        .post("http://junjian.admin.zhan-yan.cn:57658/adminApi/rpc/clean/1/start")
        .json(&req)
        .send()
        .await;

    let resp_info = response.unwrap().text().await.unwrap();
    info!("启动洗车响应: {}", resp_info);

    (StatusCode::CREATED, Json(resp_info))
}

async fn finish_clean(Path(order_no): Path<String>) -> (StatusCode, Json<String>) {
    let client = reqwest::Client::new();
    let req = json!({
        "memberId" : 1411424612450304i64,
        "orderNo" : order_no
    });
    let response = client
        .post("http://junjian.admin.zhan-yan.cn:57658/adminApi/rpc/clean/1/finish")
        .json(&req)
        .send()
        .await;

    let resp_info = response.unwrap().text().await.unwrap();
    info!("結束洗车响应: {}", resp_info);

    (StatusCode::CREATED, Json(resp_info))
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