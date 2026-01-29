#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::app::components::Header::Header;
use crate::router::PageRouter::Route;

#[component]
pub fn Main() -> Element {
    rsx! {
        div{
            class:"body__wrapper w-full h-full bg-zinc-900",
            Header {},
            main {
                class: "w-full relative",
                Outlet::<Route> {}
            },
        }
    }
}
