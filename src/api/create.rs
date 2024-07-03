use axum::response::Html;
use chrono::{DateTime, Local};

pub fn create() -> Html<String> {
    let local = Local::now().format("%A, %d %B, %Y").to_string();
    let time = Local::now().format(" %-I:%M %p").to_string();

    Html(format!(
        r#"
    <br style="opacity: 0;">
    <article class="article" style="color: black;">
      <h4 class="date">{local}</h4>
      <h2 class="article-title" contenteditable="true" data-ph="A title for the day"></h2>
      <hr class="hbar">
      <p class="article-content" contenteditable="true" data-ph="Well, what's on your mind?" data-height="100vh"></p>
    </article>
    "#
    ))
}

pub fn sidenav() -> Html<String> {
    Html(
        r#"
  <div class="option" >
    <img src="/buttons/favorite_heart.svg"></img> &nbspabout
  </div> <br> <br>
  <div class="option" onclick="window.__TAURI__.tauri.invoke('redirect', {to: 'settings'})">
    <img src="/buttons/add.svg"></img> &nbspsettings
  </div>
  "#.to_string()
    )
}
