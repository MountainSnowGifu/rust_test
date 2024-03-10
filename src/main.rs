use axum::{
    http::{HeaderValue, Method},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let frontend = async {
        let app = Router::new().route("/", get(json)).layer(
            CorsLayer::new()
                //.allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );
        serve(app, 3000).await;
    };

    let backend = async {
        let app = Router::new().route("/json", get(json)).layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            //
            // pay attention that for some request types like posting content-type: application/json
            // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
            // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                //.allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );
        serve(app, 4000).await;
    };

    tokio::join!(frontend, backend);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn html() -> impl IntoResponse {
    Html(include_str!("../front/index.html"))
}

async fn json() -> impl IntoResponse {
    // let data = Todos {
    //     data: vec![
    //         Todo {
    //             userid:1,
    //             id: 1,
    //             title: "takashi1".to_string(),
    //             completed: false,
    //         },
    //         Todo {
    //             userid:2,
    //             id: 2,
    //             title: "takashi2".to_string(),
    //             completed: false,
    //         },
    //     ],
    // };

    let data = vec![
        Todo {
            userId: 1,
            id: 1,
            title: "takashi1".to_string(),
            completed: false,
        },
        Todo {
            userId: 2,
            id: 2,
            title: "takashi2".to_string(),
            completed: false,
        },
        Todo {
            userId: 3,
            id: 3,
            title: "takashi3".to_string(),
            completed: false,
        },
    ];

    let builder = Response::builder();
    let body = Json(data);

    let body = builder
        .status(StatusCode::BAD_REQUEST)
        .header("Content-type", "application/json")
        .body(body)
        .unwrap()
        .into_body();

    println!("{:?}", body);

    body
}

#[derive(Clone, Deserialize, Serialize, Debug)]
// 構造体を定義
struct Todo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Todos {
    data: Vec<Todo>,
}
