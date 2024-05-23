#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::home::HomePage::HomePage;

#[component]
pub fn Main() -> Element {
    rsx! {
        main {
            class: "w-full relative pb-24",
            HomePage {},
        },

    }
}
