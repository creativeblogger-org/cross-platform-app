use chrono::Local;
use dioxus::prelude::*;
use dioxus_router::{use_route, Link};

use crate::{Error, structs::post::Post, API_URL, utils::get_human_date};

#[allow(non_snake_case)]
pub fn Post(cx: Scope) -> Element {
    let post = use_state(&cx, || Post::default());
    let is_loading = use_state(cx, || true);
    let route = use_route(&cx);
    let error = use_shared_state::<Error>(&cx).unwrap();

    let slug = match route.segment("slug") {
        Some(slug) => slug,
        None => {
            return render!{rsx!{Link {to: "/404"}}}
        }
    };

    use_future(cx, (), |_| {
        to_owned![post, error, slug, is_loading];
        async move {
            is_loading.set(true);
            let res = match reqwest::get(format!("{}/posts/{}", API_URL, slug)).await {
                Ok(res) => res,
                Err(e) => {
                    is_loading.set(false);
                    return error.write().0 = e.to_string();
                }
            };
            let received_post: Post = match res.json().await {
                Ok(post) => post,
                Err(e) => {
                    is_loading.set(false);
                    return error.write().0 = e.to_string();
                }
            };
            is_loading.set(false);
            post.set(received_post);
        }
    });

    render! {
        rsx! {
            div {
                class: "bg-black text-white text-center p-16 w-full min-h-screen",
                if *is_loading.get() {
                    rsx! {
                        p {
                            "Chargement du post..."
                        }
                    }
                } else if post.id == 0 && error.read().0.is_empty() {
                    rsx! {
                        Link {
                            to: "/404"
                        }
                    }
                } else if error.read().0.is_empty() {
                    rsx! {
                        div {
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