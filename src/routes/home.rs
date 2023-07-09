use chrono::Local;
use dioxus::prelude::*;
use dioxus_router::Link;

use crate::{API_URL, structs::post::PreviewPost, Error, utils::get_human_date};

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let posts = use_state(cx, || Vec::<PreviewPost>::new());
    let is_loading = use_state(cx, || false);
    let error = use_shared_state::<Error>(cx).unwrap();

    use_future(cx, (),  |_| {
        to_owned![posts, error, is_loading];
        async move {
            is_loading.set(true);
            let res = match reqwest::get(format!("{}/posts", API_URL)).await {
                Ok(res) => res,
                Err(e) => {
                    is_loading.set(false);
                    return error.write().0 = e.to_string();
                }
            };

            let received_posts: Vec<PreviewPost> = match res.json().await {
                Ok(posts) => posts,
                Err(e) => {
                    is_loading.set(false);
                    return error.write().0 = e.to_string();
                }
            };

            is_loading.set(false);
            posts.set(received_posts);
        }
    });

    render! (
        rsx! {
            div {
                class: "bg-black w-full min-h-screen",
                div {
                    class: "text-white p-4 grid grid-cols-3",
                    
                    if *is_loading.get() {
                        rsx! {
                            p {
                                "Chargement..."
                            }
                        }
                    } else if posts.len() == 0 && error.read().0.is_empty() {
                        rsx! {
                            p {
                                "Pas de post pour le moment"
                            }
                        }
                    }
                    for post in posts.iter() {
                        rsx! {
                            div {
                                class: "rounded-md bg-slate-950 p-4 m-4 text-center",
                                background_image: "url({post.image})",
                                Link {
                                    to: "/posts/{post.slug}",
                                    h1 {
                                        class: "font-bold text-2xl",
                                        post.title.to_string()
                                    }
                                    Link {
                                        to: "/@{post.author.username}",
                                        class: "hover:underline hover:font-bold",
                                        p {
                                            "@{post.author.username}"
                                        }
                                    }
                                    p {
                                        "Créé le {get_human_date(post.created_at.with_timezone(&Local))}"
                                    }
                                    if post.created_at != post.updated_at {
                                        rsx! {
                                            p {
                                                "Modifié le {get_human_date(post.updated_at.with_timezone(&Local))}"
                                            }
                                        }
                                    }
                                    p {post.description.to_string()}
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}