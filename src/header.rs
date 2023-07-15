use dioxus::prelude::*;
use dioxus_router::Link;

#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
    render! {
        rsx! {
            div {
                class: "bg-black text-white text-center p-3 font-pangolin",
                img {
                    class: "mx-auto",
                    src: "./public/images/cb_logo.png",
                    width: 100,
                    height: 100,
                    alt: "Logo de Creative Blogger",
                }
                Link {
                    to: "/",
                    class: "text-2xl",
                    "Creative Blogger"
                }
            }
        }
    }
}
