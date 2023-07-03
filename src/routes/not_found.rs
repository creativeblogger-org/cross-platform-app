use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn NotFound(cx: Scope) -> Element {
    render!(
        rsx! {
            "Page introuvable"
        }
    )
}