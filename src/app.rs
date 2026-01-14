#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod components;
use crate::router::PageRouter::Route;

static CSS: Asset = asset!("/public/assets/css/styles.css");
static TAILWIND_CSS: Asset = asset!("public/assets/css/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS },
        document::Stylesheet { href: TAILWIND_CSS},
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@20..48,100..700,0..1,-50..200"},
        Router::<Route> {}
    }
}
