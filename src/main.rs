mod routes;
mod structs;
mod utils;

use chrono::{DateTime, Local, Datelike, Timelike};
use dioxus::{prelude::*, html::h2};
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::WindowBuilder;
use dioxus_router::{Router, Route, Redirect};

use crate::routes::{home::Home, not_found::NotFound, post::Post};

const API_URL: &str = "https://api.creativeblogger.org";
const ASSETS_PATH: &str = {
    #[cfg(target_arch = "wasm32")]
    let path = ".";
    #[cfg(not(target_arch = "wasm32"))]
    let path = "public";
    path
};

struct Error(String);

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(&cx, || Error("".to_string()));
    let error = use_shared_state::<Error>(&cx).unwrap();
    let string_error = error.read().0.to_owned();

    render! (
        Router {
            h2 {
                class: "absolute text-center top-0",
                "{string_error}"
            }
            Route { to: "/", Home {} }
            Route { to: "/posts/:slug", Post {} }
            Route { to: "/404", NotFound {} }
            Redirect { from: "", to: "/404" }
        }
    )
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    dioxus_desktop::launch_cfg(
        App,
        dioxus_desktop::Config::new()
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
            .with_window(WindowBuilder::default().with_title("Creative Blogger App")),
    );
    #[cfg(target_arch = "wasm32")]
    dioxus_web::launch(App);
}