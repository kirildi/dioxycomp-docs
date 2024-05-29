#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod components;
use crate::router::PageRouter::Route;
use components::Header::Header;

#[component]
pub fn App() -> Element {
    rsx! {
        div{
            class:"bg-gray-800",
            Header {},
            Outlet::<Route> {}
        }
    }
}
