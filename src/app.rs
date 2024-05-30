#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub mod components;
use crate::router::PageRouter::Route;

#[component]
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
