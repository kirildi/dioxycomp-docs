#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod components;
use crate::router::PageRouter::Route;

#[component]
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
