#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::router::PageRouter::Route;

#[inline_props]
pub fn SideBarNav(cx: Scope) -> Element {
    let nav_style = "fixed p-4 w-full h-2/6 bg-neutral-900 lg:w-72 lg:h-full";
    let li_style = "flex-none pl-4 py-2 hover:bg-neutral-600 hover:rounded-md hover:duration-100";

    cx.render(rsx! {
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
                                    Link {
                                        class: "",
                                        to: Route::PageLoader { name: String::from("Button")},
                                          li {
                                        class: "{li_style}",

                                                      "Button"
                                                }
                                    },
                                     Link {
                                        class: " ",
                                        to: Route::PageLoader { name: String::from("Checkbox")},
                                          li {
                                                class: "{li_style}",
                                                "Checkbox"
                                          }
                                    },
                                    Link {
                                        class: " ",
                                        to: Route::PageLoader { name: String::from("Radio")},
                                    li {
                                          class: "{li_style}",
                                        "Radio"
                                          }
                                    },
                                    Link {
                                        class: " ",
                                        to: Route::PageLoader { name: String::from("Select")},
                                    li {
                                          class: "{li_style}",
                                        "Select"
                                    }
                            },
                      }
                }
          },
    })
}
