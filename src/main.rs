use std::path::Path;

use dioxus::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::WindowBuilder;
use serde::Deserialize;

const API_URL: &str = "https://api.creativeblogger.org";
const ASSETS_PATH: &str = {
    #[cfg(target_arch = "wasm32")]
    let path = ".";
    #[cfg(not(target_arch = "wasm32"))]
    let path = "public";
    path
};

#[derive(Debug, Deserialize)]
struct PreviewPost {
    title: String,
    description: String
}

fn app(cx: Scope) -> Element {
    let posts = use_state(cx, || Vec::new());

    use_future(cx, (),  |_| {
        to_owned![posts];
        async move {
            posts.set(reqwest::get(format!("{}/posts", API_URL)).await.unwrap().json::<Vec<PreviewPost>>().await.unwrap());
        }
    });

    println!("{} {}", ASSETS_PATH, Path::new(ASSETS_PATH).to_str().unwrap());

    render! (
        div {
            class: "bg-black text-white",
            for post in posts.iter() {
                div {
                    h1 {
                        post.title.to_string()
                    }
                    p {post.description.to_string()}
                }
            }
        }
    )
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch_cfg(
        app,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
            .with_window(WindowBuilder::default().with_title("Creative Blogger App")),
    );
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(app);
}