use dioxus::prelude::*;
use dioxus_router::Link;

#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
    render! {
        rsx! {
            div {
                class: "bg-black text-center p-3 font-pangolin",
                img {
                    class: "mx-auto",
                    src: "./public/images/cb_logo.png",
                    width: 100,
                    height: 100,
                    alt: "Logo de Creative Blogger",
                }
                Link {
                    to: "/",
                    class: "text-4xl navbar-link",
                    "Creative Blogger"
                }
                div {
                    class: "flex justify-between",
                    p {
                        class: "text-xl navbar-link",
                        "Plus"
                    }
                    div {
                        class: "text-xl",
                        Link {
                            to: "/login",
                            class: "p-2 navbar-link border-teal-500 hover:border-b-2",
                            "Se connecter"
                        }
                        Link {
                            to: "/register",
                            class: "p-2 navbar-link border-teal-500 hover:border-b-2",
                            "Créer un compte"
                        }
                    }
                }
            }
        }
    }
}
