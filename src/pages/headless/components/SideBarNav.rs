#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::headless::components::Badge::{Badge, BadgeKind, BadgeProps};
use crate::router::PageRouter::Route;

pub fn SideBarNav() -> Element {
    let nav_style = "fixed p-4 w-full h-2/6 bg-neutral-900 lg:w-72 lg:h-full";
    let li_style = "flex-none px-4 py-2 hover:bg-neutral-600 hover:rounded-md hover:duration-100";

    let links = ["Button", "Checkbox", "Radio", "Select"];
    let badge_class = "";
    let li_link_loader = links.iter().map(|link_key| {
        let badge_prop = BadgeProps {
            id: Some(String::from("0")),
            value: None,
            kind: Some(BadgeKind::Dev),
            class_name: Some(String::from("px-3 py-1 rounded text-xs")),
            styling: None,
        };
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
                        Badge {   },
                  }
              }
        }}
    });

    rsx! {
          nav {
              id: "sidebar",
                class: "{nav_style}",
                details {
                    class: "group",
                    open: "true",
                    summary {
                          class: "p-3 w-64 h-12 bg-zinc-800 rounded-xl group-open:rounded-b-none font-semibold",
                          "Components",
                    },
                    ul {
                          class: "pb-3 bg-gray-800/20 rounded-b-xl",

                      }
                }
          },
    }
}
