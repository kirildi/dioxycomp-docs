#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::router::PageRouter::Route;

pub fn SideBarNav() -> Element {
    let nav_style = "p-4 bg-neutral-900";
    let li_style = "px-4 py-2 hover:bg-neutral-600 hover:rounded-md hover:duration-100";

    let links = ["Button", "Checkbox", "Radio", "Select"];
    let li_link_loader = links.iter().map(|link_key| {
        rsx! {
            Link {
                class: "",
                to: Route::PageLoader { name: String::from(*link_key)},
                li {
                class: "{li_style}",
                    div {
                        class: "flex justify-between w-full",
                        span {
                            "{link_key}"
                        }
                    }
                }
            }
        }
    });

    rsx! {
        nav {
            id: "sidebar",
            class: "{nav_style}",
            details {
                class: "group h-2/6 max-w-72",
                open: "true",
                summary {
                        class: "p-3 lg:w-72 h-12 bg-zinc-800 rounded-xl font-semibold",
                        "Components",
                    },
                ul {
                    class: "pb-3 rounded-b-xl",
                    {li_link_loader}
                }
            }
        },
    }
}
