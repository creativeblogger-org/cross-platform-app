use chrono::Local;
use dioxus::prelude::*;
use dioxus_router::Link;

use crate::{API_URL, structs::post::PreviewPost, Error, utils::get_human_date};

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    let posts = use_state(cx, || Vec::<PreviewPost>::new());
    let error = use_shared_state::<Error>(&cx).unwrap();

    use_future(cx, (),  |_| {
        to_owned![posts, error];
        async move {
            let res = match reqwest::get(format!("{}/posts", API_URL)).await {
                Ok(res) => res,
                Err(e) => {
                    error.write().0 = e.to_string();
                    return ();
                }
            };

            let received_posts: Vec<PreviewPost> = match res.json().await {
                Ok(posts) => posts,
                Err(e) => {
                    error.write().0 = e.to_string();
                    return ();
                }
            };

            posts.set(received_posts);
        }
    });

    render! (
        rsx! {
            div {
                class: "bg-black w-screen min-h-screen",
                div {
                    class: "text-white p-4 grid grid-cols-3",
                    for post in posts.iter() {
                        div {
                            class: "rounded-md bg-slate-950 p-4 m-4 text-center",
                            Link {
                                to: "/posts/{post.slug}",
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
        }
    )
}