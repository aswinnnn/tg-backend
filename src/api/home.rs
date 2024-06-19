use axum::response::Html;

pub fn home() -> Html<String> {
    Html(format!(r#"
    <div class="post-bg" data-src="/assets/girl-garden-border-cover.svg">
      <div class="post-content">
        <h4>this is a post title </h4><br>
        <p> this is a post i made a long time ago about blah blah blah...</p>
      </div>
    </div>

    <br style="opacity: 0;">
    <div class="post-bg" data-src="/assets/MaidenInGarden.svg">
      <div class="post-content">
        <h4>this is a post title </h4><br>
        <p> this is a post i made a long time ago about blah blah blah...</p>
      </div>
    </div>

    <br style="opacity: 0;">
    <div class="post-bg" data-src="/assets/macabre.svg">
      <div class="post-content">
        <h4>this is a post title </h4><br>
        <p> this is a post i made a long time ago about blah blah blah...</p>
      </div>
    </div>
    "#))
}