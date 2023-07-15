mod header;
mod routes;
mod structs;
mod utils;

use dioxus::prelude::*;
use dioxus_router::{Redirect, Route, Router};

use crate::{
    header::Header,
    routes::{home::Home, not_found::NotFound, post::Post},
};

const API_URL: &str = "http://localhost:3333";

fn init_logging() {
    #[cfg(not(target_os = "android"))]
    simple_logger::SimpleLogger::new().init().unwrap();
    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    )
}

#[derive(PartialEq)]
struct Error(String);

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(&cx, || Error("".to_string()));
    let error = use_shared_state::<Error>(&cx).unwrap();
    let string_error = error.read().0.to_owned();

    render! (
        Router {
            if !string_error.is_empty() {
                rsx! {
                    div {
                        class: "text-center absolute w-full top-0 p-3 bg-white opacity-75",
                        h2 {
                            class: "text-red-600 font-bold",
                            "{string_error}"
                        }
                    }
                }
            }
            Header {}
            Route { to: "/", Home {} }
            Route { to: "/posts/:slug", Post {} }
            Route { to: "/404", NotFound {} }
            Redirect { from: "", to: "/404" }
        }
    )
}

pub fn start_app() {
    init_logging();

    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_window(dioxus_desktop::WindowBuilder::default().with_title("Creative Blogger"))
            //.with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css"><link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Pangolin&display=swap">"#.to_string()),
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css"><style>@font-face {font-display: swap; font-family: "Pangolin"; font-size: normal; font-weight: normal;src: url('./public/fonts/pangolin-v11-cyrillic_cyrillic-ext_latin_latin-ext_vietnamese-regular.woff2') format('woff2');}</style>"#.to_string()),
    );
}
