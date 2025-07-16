use anyhow::{Context, Result};
use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::{Html, Json},
    routing::{get, post},
    Form, Router,
};
use log::info;
use serde::Deserialize;
use serde_json::{json, Value};
// use std::net::SocketAddr;
use crate::common;
use crate::use_sqlite;

#[derive(Deserialize)]
struct LogIn {
    user: String,
}
#[derive(Deserialize)]
struct UserForm {
    username: String,
    password: String,
}
#[derive(Deserialize)]
struct UserJson {
    username: String,
    password: String,
}

pub async fn run_server() -> Result<()> {
    let app = Router::new()
        // 首页
        .route("/", get(root))
        .route("/login", get(log_in)) // /login?user=aaa
        // 用户相关路由
        .route("/users", get(list_users))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user))
        .route("/users_post", post(post_user))
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

async fn log_in(Query(params): Query<LogIn>, headers: HeaderMap) -> Json<Value> {
    info!("log in param user: {}", params.user);

    //  let user_agent = headers.get("User-Agent")        // Option<&HeaderValue>
    //     .ok_or(common::MyError::MissingValue)?      // 转为 Result<&HeaderValue, MyError>
    //     .to_str()                                   //Result<&str, ToStrError>
    //     .map_err(|_| common::MyError::InvalidValue)?;       // 最终 Result<&str, MyError>
    // info!("User-Agent: {}", user_agent);

    let user_agent = headers
        .get("User-Agent") // Option<&HeaderValue>
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or("");
    info!("User-Agent: {}", user_agent);

    // let accept = headers.get("Accept").unwrap().to_str().unwrap();
    let accept = headers
        .get("Accept") // Option<&HeaderValue>
        .and_then(|hv| hv.to_str().ok()) // .ok()‌将 Result 转换为 Option 类型：成功时（Ok）→ Some(&str) 失败时（Err）→ None
        .unwrap_or("");
    info!("Accept: {}", accept);

    match use_sqlite::query_data(&params.user) {
        Ok(data) if !data.is_empty() => {
            if data == "aaa_value" {
                info!("查询ok");
                Json(json!([
                    {"ret": "ok"}
                ]))
            } else {
                info!("非预期值: {}", data);
                Json(json!([{"ret": "unexpected"}]))
            }
        }
        Ok(_) => {
            info!("null");
            Json(json!([
                {"ret": "null."}
            ]))
        }
        Err(e) => {
            info!("错误：{}", e);
            Json(json!([
                {"ret": "failed."}
            ]))
        }
    }
}

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

async fn create_user(Form(form): Form<UserForm>) -> Json<Value> {
    info!(
        "post1 Received username: {}, password: {}",
        form.username, form.password
    );
    Json(json!({"status": "user created"}))
}
async fn post_user(Json(json): Json<UserJson>) -> Json<Value> {
    info!(
        "post2 Received username: {}, password: {}",
        json.username, json.password
    );
    Json(json!({"status": "user post"}))
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
