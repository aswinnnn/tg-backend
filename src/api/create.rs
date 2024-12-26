use axum::response::{Html, IntoResponse};
use chrono::{DateTime, Local};

pub async fn create() -> impl IntoResponse {
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
    <script>
      debounceUpdateBuffer()
    </script>
    "#
    ))
}

pub async fn sidenav() -> impl IntoResponse {
    Html(
        r##"
  <div class="option" onclick="window.__TAURI__.tauri.invoke('redirect', {to: 'about'})">
<svg xmlns="http://www.w3.org/2000/svg" height="30px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="m480-120-58-52q-101-91-167-157T150-447.5Q111-500 95.5-544T80-634q0-94 63-157t157-63q52 0 99 22t81 62q34-40 81-62t99-22q94 0 157 63t63 157q0 46-15.5 90T810-447.5Q771-395 705-329T538-172l-58 52Zm0-108q96-86 158-147.5t98-107q36-45.5 50-81t14-70.5q0-60-40-100t-100-40q-47 0-87 26.5T518-680h-76q-15-41-55-67.5T300-774q-60 0-100 40t-40 100q0 35 14 70.5t50 81q36 45.5 98 107T480-228Zm0-273Z"/></svg>
     &nbspabout
  </div> <br> <br>
  <div class="option" onclick="window.__TAURI__.tauri.invoke('redirect', {to: 'settings'})">
<svg xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor"><path d="m370-80-16-128q-13-5-24.5-12T307-235l-119 50L78-375l103-78q-1-7-1-13.5v-27q0-6.5 1-13.5L78-585l110-190 119 50q11-8 23-15t24-12l16-128h220l16 128q13 5 24.5 12t22.5 15l119-50 110 190-103 78q1 7 1 13.5v27q0 6.5-2 13.5l103 78-110 190-118-50q-11 8-23 15t-24 12L590-80H370Zm70-80h79l14-106q31-8 57.5-23.5T639-327l99 41 39-68-86-65q5-14 7-29.5t2-31.5q0-16-2-31.5t-7-29.5l86-65-39-68-99 42q-22-23-48.5-38.5T533-694l-13-106h-79l-14 106q-31 8-57.5 23.5T321-633l-99-41-39 68 86 64q-5 15-7 30t-2 32q0 16 2 31t7 30l-86 65 39 68 99-42q22 23 48.5 38.5T427-266l13 106Zm42-180q58 0 99-41t41-99q0-58-41-99t-99-41q-59 0-99.5 41T342-480q0 58 40.5 99t99.5 41Zm-2-140Z"/></svg>
     &nbspsettings
  </div>
  "##
        .to_string(),
    )
}
