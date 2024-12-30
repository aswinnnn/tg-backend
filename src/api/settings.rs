use std::process::exit;

use crate::config;
use axum::{response::{Html, IntoResponse}, Json};

pub async fn read() -> impl IntoResponse {
    match config::json::read_config() {
        Ok(conf) => {
            Json(conf)
        },
        Err(e) => {eprintln!("[SETTINGS-READ] {e}"); exit(1)},
    }
}

pub async fn general() -> impl IntoResponse {
    let res: String = String::new();

    match config::json::read_config() {
        Ok(conf) => Html(format!(
            r#"
        <h2> General Settings</h2>
        <div class ="notice">sorry, no general settings yet, most of the settings are in the appearance section.<br>
        this section is for encryption,password protection, auto-lock after inactivity, cloud sync, etc <br>
        which is yet to be added since this is a beta release.</div>
    "#
        )),
        Err(e) => Html(format!(
            r#"<div class="error" style="background-color: yellow;">An error occurred in read_config(): <br> {e}</div>"#
        )),
    }
}

pub async fn appearance() -> impl IntoResponse {
    let res: String = String::new();

    match config::json::read_config() {
        Ok(conf) => Html(format!(
            r#"
        <h2>Theme</h2>
        <h3>- Light mode</h3>
        <div class="option">
                 <label for="theme.light.background-color">background color</label>
                 <textarea id="theme.light.background-color" name="theme.light.background-color" maxlength="10">{}</textarea>
        </div>
        <div class="option">
                 <label for="theme.light.text-color">text color</label>
                 <textarea id="theme.light.text-color" name="theme.light.text-color" maxlength="10">{}</textarea>
        </div>
        <h3>- Dark mode</h3>
        <div class="option">
                 <label for="theme.dark.background-color">background color</label>
                 <textarea id="theme.dark.background-color" name="theme.light.background-color" maxlength="10">{}</textarea>
        </div>
        <div class="option">
                 <label for="theme.dark.text-color">text color</label>
                 <textarea id="theme.dark.text-color" name="theme.dark.text-color" maxlength="10">{}</textarea>
        </div>
        <hr>
        <h2>Emoji</h2>
        <div class="option">
                 <label for="">home button</label>
                 <textarea id="home.emoji" name="home.emoji" maxlength="10">{}</textarea>
        </div>
        <div class="option">
                 <label for="">create button</label>
                 <textarea id="create.emoji" name="create.emoji" maxlength="10">{}</textarea>
        </div>
        <script>loadlisteners()</script>
    "#,
            conf.get("theme.light.background-color").unwrap().as_str().unwrap().to_string(),
            conf.get("theme.light.text-color").unwrap().as_str().unwrap().to_string(),
            conf.get("theme.dark.background-color").unwrap().as_str().unwrap().to_string(),
            conf.get("theme.dark.text-color").unwrap().as_str().unwrap().to_string(),
            conf.get("home.emoji").unwrap().as_str().unwrap().to_string(),
            conf.get("create.emoji").unwrap().as_str().unwrap().to_string() 
            //  this as_str() hack is done to avoid matching Value (or dealing with those quotation marks in output)
        )),
        Err(e) => Html(format!(
            r#"<div class="error" style="background-color: yellow;">An error occurred in read_config(): <br> {e}</div>"#
        )),
    }
}
