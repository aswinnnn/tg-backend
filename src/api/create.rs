use axum::response::Html;
use time::{self, format_description};

pub fn create() -> Html<String> {
  let format = format_description::parse("
  [weekday], [month], [year] [hour]:[minute]
  ").expect("time format failed");
  let t = time::OffsetDateTime::now_utc().format(&format).expect("time format failed");
  
  Html(format!(r#"
    <br style="opacity: 0;">
    <article class="article">
      <h4 class="date">{t}</h4>
      <h2 class="article-title" contenteditable="true" data-ph="A title for the day"></h2>
      <hr class="hbar">
      <p class="article-content" contenteditable="true" data-ph="Well, what's on your mind?" data-height="100vh"></p>
    </article>"#))

}