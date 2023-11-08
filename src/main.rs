use axum;
use axum::{routing::get, Router, extract::Json};
use std::net::SocketAddr;
use std::time::Duration;
use axum::handler::Handler;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::post;
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};
use soar::infrastructure::response::response;
use log::warn;
use soar::infrastructure::response::response::ResultVO;


async fn fallback(uri: Uri) -> impl IntoResponse {
    let msg = format!("没有当前资源： {}，请核对请求路径", uri);
    warn!("{}",msg);
    response::ResultVO::<String>::from_msg_fail(&msg)
        .resp_json()
}

#[tokio::main]
async fn main() {
    //初始化
    soar::init_config().await;
    //repo层的初始化
    soar::init_repository().await;

    //cors
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any)
        .max_age(Duration::from_secs(60) * 10);

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar))
        .route("/result", post(result_example))
        .layer(cors)
        .fallback(fallback);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8090));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct CreateUser {
    username: String,
}

//Json(payload): Json<CreateUser>
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<CreateUser>) {
    (StatusCode::CREATED, Json(payload))
}

async fn root() -> String {
    String::from("hello axum")
}

async fn get_foo() -> String {
    String::from("get请求的foo")
}

async fn post_foo() -> String {
    String::from("post请求的foo")
}

async fn foo_bar() -> String {
    String::from("foo:bar")
}


async fn result_example() -> impl IntoResponse {
    let msg =String::from("asdfasdf");
    ResultVO::<String>::from_msg_fail(&msg).resp_json()
}