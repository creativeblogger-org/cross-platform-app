use chrono::Local;
use dioxus::prelude::*;
use dioxus_router::Link;

use crate::{API_URL, structs::post::PreviewPost, get_human_date};

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let posts = use_state(cx, || Vec::new());

    use_future(cx, (),  |_| {
        to_owned![posts];
        async move {
            posts.set(reqwest::get(format!("{}/posts", API_URL)).await.unwrap().json::<Vec<PreviewPost>>().await.unwrap());
        }
    });

    render! (
        rsx! {
            div {
                class: "bg-black text-white p-4 grid grid-cols-3",
                for post in posts.iter() {
                    Link {
                        to: "/posts/{post.slug}",
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
            }
        }
    )
}