use anyhow::{Context, Result};
use axum::{
    extract::Path,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
// use std::net::SocketAddr;

pub async fn run_server() -> Result<()> {
    let app = Router::new()
        // 首页
        .route("/", get(root))
        // 用户相关路由
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        // 产品相关路由
        .route("/products", get(list_products))
        .route("/products/:id", get(get_product))
        // 健康检查
        .route("/health", get(health_check))
        // 演示不同响应类型
        .route("/html", get(html_response))
        .route("/json", get(json_response));

    // 启动服务器
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let addr = "127.0.0.1:3000".parse()?;
    println!("Server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("Failed to start server")?;

    Ok(())
}

// 处理器函数

async fn root() -> Html<&'static str> {
    Html("<h1>Welcome to the Rust Web Server!</h1>")
}

async fn list_users() -> Json<Value> {
    Json(json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]))
}

async fn get_user(Path(id): Path<u32>) -> Json<Value> {
    Json(json!({"id": id, "name": format!("User {}", id)}))
}

async fn create_user() -> Json<Value> {
    Json(json!({"status": "user created"}))
}

async fn list_products() -> Json<Value> {
    Json(json!([
        {"id": 1, "name": "Rust Book"},
        {"id": 2, "name": "Axum Guide"}
    ]))
}

async fn get_product(Path(id): Path<u32>) -> Json<Value> {
    Json(json!({"id": id, "name": format!("Product {}", id)}))
}

async fn health_check() -> &'static str {
    "OK"
}

async fn html_response() -> Html<&'static str> {
    Html("<html><body><h1>HTML Response</h1></body></html>")
}

async fn json_response() -> Json<Value> {
    Json(json!({"message": "This is a JSON response"}))
}
