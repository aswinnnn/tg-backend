use std::path::PathBuf;

use axum::http::HeaderValue;
use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
mod analysis;
mod api;
pub mod config;
pub mod db;
pub mod journal;
mod routes;

pub async fn start(resource: PathBuf) {
    // cors so tauri can fetch without trouble
    println!("[TG-BACKEND] found resource directory: {:#?}", resource);
    let origins = [
        "tauri://localhost".parse::<HeaderValue>().unwrap(),
        "http://localhost:1420".parse::<HeaderValue>().unwrap(),
        "https://tauri.localhost".parse::<HeaderValue>().unwrap(),
    ];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "there is nothing here." }))
        .route("/api/create", get(api::create::create))
        .route("/api/home", get(api::home::home))
        .route("/api/create/sidenav", get(api::create::sidenav))
        .route("/api/posts/options", get(api::posts::options))
        .nest_service("/src", ServeDir::new(&resource))
        .nest_service("/assets", ServeDir::new(&resource.join("assets")))
        .nest_service("/intro", ServeDir::new(&resource.join("intro")))
        .nest_service("/create", ServeDir::new(&resource.join("create")))
        .nest_service("/buttons", ServeDir::new(&resource.join("buttons")))
        .nest_service("/settings", ServeDir::new(&resource.join("settings")))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    println!("[TG-BACKEND] server starting...");
    axum::serve(listener, app).await.unwrap();
}
