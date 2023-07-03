mod routes;
mod structs;
mod utils;

use chrono::{DateTime, Local, Datelike, Timelike};
use dioxus::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::WindowBuilder;
use dioxus_router::{Router, Route, Redirect};

use crate::routes::{home::Home, not_found::NotFound};

const API_URL: &str = "https://api.creativeblogger.org";
const ASSETS_PATH: &str = {
    #[cfg(target_arch = "wasm32")]
    let path = ".";
    #[cfg(not(target_arch = "wasm32"))]
    let path = "public";
    path
};

fn get_human_date(timestamp: DateTime<Local>) -> String {
    format!("{:02}/{:02}/{} à {:02}:{:02}:{:02}", timestamp.day(), timestamp.month(), timestamp.year(), timestamp.hour(), timestamp.minute(), timestamp.second())
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    render! (
        Router {
            Route { to: "/", Home {} }
            Route { to: "/404", NotFound {}}
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