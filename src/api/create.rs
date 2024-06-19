use axum::response::Html;
pub fn create() -> Html<String> {
    Html(format!(r#"
    <br style="opacity: 0;">
    <article class="article">
      <h4 class="date">Friday, May 3, 2024</h4>
      <h2 class="article-title" contenteditable="true" data-ph="A title for the day"></h2>
      <hr class="hbar">
      <p class="article-content" contenteditable="true" data-ph="Well, what's on your mind?" data-height="100vh"></p>
    </article>"#))

}