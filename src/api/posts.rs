
use axum::response::{Html, IntoResponse};

pub async fn options() -> impl IntoResponse {
    r#"<img src="/buttons/more_vert.svg"></img> 
    <img src="/buttons/delete.svg" onclick="post_delete()"></img>
    <img src="/buttons/settings.svg" onclick="post_settings()"></img>
    "#
}