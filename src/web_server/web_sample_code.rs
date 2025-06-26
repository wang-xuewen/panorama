use axum::{
    extract::Path,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 集中定义所有路由
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
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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

use axum::{
    extract::Path,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建新线程来运行Web服务器
    let server_thread = thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let app = Router::new()
                .route("/", get(root))
                .route("/users", get(list_users))
                .route("/users/:id", get(get_user))
                .route("/users", post(create_user))
                .route("/products", get(list_products))
                .route("/products/:id", get(get_product))
                .route("/health", get(health_check))
                .route("/html", get(html_response))
                .route("/json", get(json_response));

            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            println!("Server starting on {}", addr);

            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
    });

    println!("Main thread continues to run while server is up...");

    // 主线程可以做其他工作
    for i in 1..=5 {
        println!("Main thread working: {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    // 等待服务器线程结束（实际应用中可能需要更复杂的处理）
    server_thread.join().unwrap();
}

// 处理器函数保持不变...
async fn root() -> Html<&'static str> {
    Html("<h1>Welcome to the Rust Web Server!</h1>")
}

async fn list_users() -> Json<Value> {
    Json(json!([
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]))
}

// ...其他处理器函数保持不变

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let server_thread = thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            // ...服务器初始化代码...

            let server = axum::Server::bind(&addr).serve(app.into_make_service());

            let graceful = server.with_graceful_shutdown(async {
                while r.load(Ordering::Relaxed) {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            });

            if let Err(e) = graceful.await {
                eprintln!("server error: {}", e);
            }
        });
    });

    // 主线程等待Ctrl+C信号
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        signal::ctrl_c().await.unwrap();
        running.store(false, Ordering::Relaxed);
        println!("Shutting down gracefully...");
    });

    server_thread.join().unwrap();
}
