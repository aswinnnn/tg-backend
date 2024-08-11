use std::{fs, io::Read};

use axum::response::{Html, IntoResponse};
use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::{
    config::Configuration,
    journal::{
        store::{store_path, Store},
        Media, MediaType,
    },
};

pub async fn home() -> impl IntoResponse {
    Html(journals().await.to_string())
}

async fn journals() -> String {
    if Configuration::exists() {
        let mut js = vec![String::new()];

        if let Ok(store) = Store::new() {
            for i in store.index {
                if let Ok(id) = uuid::Uuid::parse_str(&i) {
                    js.push(generate_card(id).await);
                }
            }
        }

        let mut o = js.join(r#"<br style="opacity: 0;">"#);
        o.push_str(
            r#"<script>

    function setupPostClickHandlers() {
      const postElements = document.querySelectorAll('.post-bg');

      postElements.forEach(post => {
        console.log('you here?')
        post.addEventListener('click', async function () {
          const postId = post.getAttribute('data-id');

          try {
            await window.__TAURI__.invoke('fill_post', { postId: postId });
          } catch (error) {
            console.error('Error calling Tauri function:', error);
          }
        });
      });
    }
        setupPostClickHandlers();</script>"#,
        );
        o
    } else {
        "go ahead, write something down.".into()
    }
}

// fn journals() -> String {
//     if Configuration::exists() {
//         let mut js = vec![String::new()];

//         for i in Store::new().unwrap().index {
//             let id = uuid::Uuid::parse_str(&i).unwrap();
//             js.push(generate_card(id));
//         }

//         js.join(r#"<br style="opacity: 0;">"#)
//     } else {
//         "go ahead and write something down.".into()
//     }
// }

async fn generate_card(id: Uuid) -> String {
    let j = Store::get_journal(id.as_bytes().to_vec()).unwrap();

    let content: String = j.buffer.chars().take(64).collect();
    let wp = match Media::get(id.as_bytes().to_vec(), MediaType::Wallpaper(String::new())) {
        MediaType::Wallpaper(w) => w,
        _ => String::new(),
    };

    new_post(wp, j.buffer_title, content, &id.to_string()).await
}
async fn new_post(wallpaper: String, title: String, content: String, id_str: &str) -> String {
    let template = format!(
        r#"
    <div class="post-bg" data-src="{wallpaper}" data-id="{id_str}">
    <div class="post-menu"><img src="/buttons/menu.svg"></img></div>
      <div class="post-content">
        <h4>{title} </h4><br>
        <p> {content}...</p>
      </div>
    </div>"#
    );

    template
}
