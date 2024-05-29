#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::router::PageRouter::Route;

#[component]
pub fn Nav() -> Element {
    rsx! {
        nav {
            class: "h-24 p-8 flex gap-8 text-xl mr-0 order-last",

            Link {
                class: "hover:px-4 hover:bg-gray-600 hover:rounded hover:duration-100",
                to: Route::Main {},
                "Home"
            },
            Link {
                class: "hover:px-4 hover:bg-gray-600 hover:rounded hover:duration-100",
                to: Route::PageLoader { name: String::from("Button") },
                "Headless"
            },
        }
    }
}
