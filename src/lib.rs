use axum::http::HeaderValue;
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use tower_http::cors::{Any, CorsLayer};
mod api;
mod config;
mod routes;
mod journal;

pub async fn start() {
    // cors so tauri can fetch without trouble
    let origins = ["tauri://localhost".parse::<HeaderValue>().unwrap(), "http://localhost:1420".parse::<HeaderValue>().unwrap()];
    let cors = CorsLayer::new()
                            .allow_origin(origins)
                            .allow_methods(Any)
                            .allow_headers(Any);

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "there is nothing here." }))
                         .route("/api/create", get(api::create::create()))
                         .route("/api/home", get(api::home::home()))
                         .nest_service("/src", ServeDir::new("/home/aswin/projects/thought-garden/src"))
                         .nest_service("/assets", ServeDir::new("/home/aswin/projects/thought-garden/src/assets"))
                         .nest_service("/intro", ServeDir::new("/home/aswin/projects/thought-garden/src/intro"))
                         .nest_service("/create", ServeDir::new("/home/aswin/projects/thought-garden/src/create"))
                         .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    println!("[TG-BACKEND] server starting...");
    axum::serve(listener, app).await.unwrap();
}