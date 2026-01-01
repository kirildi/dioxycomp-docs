#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::app::components::Header::Header;
use crate::router::PageRouter::Route;

#[component]
pub fn Main() -> Element {
    rsx! {
        div{
            class:"bg-gray-800",
            Header {},
            main {
                class: "w-full relative pb-24",
                Outlet::<Route> {}
            },
        }
    }
}
