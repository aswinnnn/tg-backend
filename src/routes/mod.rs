use axum::{routing::get, Router, response::Html};
use tower_http::services::ServeDir;


pub fn routes() -> Router {
    Router::new()
            .nest_service("/intro", serve_intro())
}

fn serve_intro() -> ServeDir {
    ServeDir::new("/mydir/")
}