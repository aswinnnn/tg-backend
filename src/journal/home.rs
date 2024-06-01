use axum::response::Html;

pub fn home() -> Html<String> {
    Html(format!("
    <h1>if i gotta slap a pussy ass nigga imma make it look sexy. </h1>
    "))
}