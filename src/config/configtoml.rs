#[derive(Debug)]
struct ConfigToml {
    theme: Theme,
    media: Media
}

struct Theme {
    light: Colors,
    dark: Colors
}

struct Media {
    wallpaper: String,
    font: String,
    font_size: u64
}

struct Colors {
    bg: String,
    article_bg: String,
    text: String,
    title_text: String
}


impl ConfigToml  {
    pub fn new() {

    }
}