use chrono::{DateTime, Utc, Local, Datelike, Timelike};
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
    id: u128,
    title: String,
    slug: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    author: Author,
    description: String,
    //changement bientôt en Vec<String>
    tags: String,
    has_permission: bool
}

#[derive(Debug, Deserialize)]
struct Author {
    id: u128,
    username: String,
    permission: u8
}

fn get_human_date(timestamp: DateTime<Local>) -> String {
    format!("{:02}/{:02}/{} à {:02}:{:02}:{:02}", timestamp.day(), timestamp.month(), timestamp.year(), timestamp.hour(), timestamp.minute(), timestamp.second())
}

fn app(cx: Scope) -> Element {
    let posts = use_state(cx, || Vec::new());

    use_future(cx, (),  |_| {
        to_owned![posts];
        async move {
            posts.set(reqwest::get(format!("{}/posts", API_URL)).await.unwrap().json::<Vec<PreviewPost>>().await.unwrap());
        }
    });

    render! (
        div {
            class: "bg-black text-white p-4 grid grid-cols-3",
            for post in posts.iter() {
                div {
                    class: "rounded-md bg-slate-950 p-4 m-4",
                    h1 {
                        class: "font-bold text-2xl",
                        post.title.to_string()
                    }
                    p {format!("@{}", post.author.username)}
                    p {format!("Créé le {}", get_human_date(post.created_at.with_timezone(&Local)))}
                    p {format!("Modifié le {}", get_human_date(post.updated_at.with_timezone(&Local)))}
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