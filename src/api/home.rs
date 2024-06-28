use std::{fs, io::Read};

use axum::response::Html;
use uuid::Uuid;

use crate::{
    config::Configuration,
    journal::{
        store::{store_path, Store},
        Media, MediaType,
    },
};

pub fn home() -> Html<String> {
    Html(journals())
}

fn journals() -> String {
    if Configuration::exists() {
        let mut js = vec![String::new()];

        for i in Store::new().unwrap().index {
            let id = uuid::Uuid::parse_str(&i).unwrap();
            js.push(generate_card(id));
        }

        js.join(r#"<br style="opacity: 0;">"#)
    } else {
        return String::from("no config yet");
    }
}

fn generate_card(id: Uuid) -> String {
    let j = Store::get_journal(id.as_bytes().to_vec()).unwrap();

    let content: String = j.buffer.chars().take(64).collect();
    let wp = match Media::get(id.as_bytes().to_vec(), MediaType::Wallpaper(String::new())) {
        MediaType::Wallpaper(w) => w,
        _ => String::new(),
    };

    new_post(wp, j.buffer_title, content)
}
fn new_post(wallpaper: String, title: String, content: String) -> String {
    let template = format!(
        r#"
    <div class="post-bg" data-src="{wallpaper}">
      <div class="post-content">
        <h4>{title} </h4><br>
        <p> {content}...</p>
      </div>
    </div>"#
    );

    template
}
