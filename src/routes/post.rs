use chrono::Local;
use dioxus::prelude::*;
use dioxus_router::{use_route, Link};

use crate::{Error, structs::post::Post, API_URL, utils::get_human_date};

#[allow(non_snake_case)]
pub fn Post(cx: Scope) -> Element {
    let post = use_state(&cx, || Post::default());
    let route = use_route(&cx);
    let error = use_shared_state::<Error>(&cx).unwrap();

    let slug = match route.segment("slug") {
        Some(slug) => slug,
        None => {
            return render!{rsx!{Link {to: "/404"}}}
        }
    };

    use_future(cx, (), |_| {
        to_owned![post, error, slug];
        async move {
            let res = match reqwest::get(format!("{}/posts/{}", API_URL, slug)).await {
                Ok(res) => res,
                Err(e) => {
                    error.write().0 = e.to_string();
                    return;
                }
            };
            let received_post: Post = match res.json().await {
                Ok(post) => post,
                Err(e) => {
                    error.write().0 = e.to_string();
                    return;
                }
            };
            post.set(received_post);
        }
    });

    render! {
        rsx! {
            if !post.author.username.is_empty() {
                rsx! {
                    div {
                        class: "bg-black text-white text-center p-16 w-screen min-h-screen",
                        div {
                            class: "",
                            h1 {
                                class: "text-3xl font-bold",
                                "{post.title}"
                            }
                            Link {
                                to: "/@{post.author.username}",
                                "@{post.author.username}"
                            }
                            p {format!("Créé le {}", get_human_date(post.created_at.with_timezone(&Local)))}
                            if post.created_at != post.updated_at {
                                rsx! {
                                    p {
                                        format!("Modifié le {}", get_human_date(post.updated_at.with_timezone(&Local)))
                                    }
                                }
                            }
                            div {
                                class: "text-left",
                                p {
                                    "{post.content}"
                                }
                            }
                            p {
                                class: "text-xl",
                                "Commentaires"
                            }
                            div {
                                class: "comments text-left",
                                for comment in post.comments.iter() {
                                    div {
                                        p {
                                            "{comment.author.username}"
                                        }
                                        p {
                                            "{comment.content}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}