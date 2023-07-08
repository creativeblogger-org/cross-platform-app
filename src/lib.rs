mod routes;
mod structs;
mod utils;

use dioxus::prelude::*;
use dioxus_desktop::WindowBuilder;
use dioxus_router::{Router, Route, Redirect};

use crate::routes::{home::Home, post::Post, not_found::NotFound};

const API_URL: &str = "https://api.creativeblogger.org";

fn init_logging() {
    simple_logger::SimpleLogger::new().init().unwrap();
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
            div {
                class: "text-center absolute top-0",
                h2 {
                    class: "text-red-600",
                    "{string_error}"
                }
            }
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
            .with_window(WindowBuilder::default().with_title("Creative Blogger"))
            .with_custom_head(r#"<link rel="stylesheet" href="public/tailwind.css">"#.to_string())
    );
}