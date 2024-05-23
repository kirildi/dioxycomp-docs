#![allow(non_snake_case)]
use crate::router::PageRouter::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

pub fn HeadlessCard() -> Element {
    rsx! {
        // Card container
        div {
            class: "relative w-96 h-[24em] mx-auto rounded-2xl bg-gray-800 border-2 border-gray-300 border-solid cursor-pointer shadow-[0_0_2rem_0.3rem_rgba(99,102,241,0.6)] hover:shadow-[0_0_4em_0.6em_rgba(99,102,241,0.9)]",
            // Card main content
            Link {
                to: Route::PageLoader { name: String::from("Button") },
                div {
                    class: "h-64 p-4 border-b-2 border-gray-300 text-center",
                    span {
                        class: "material-symbols-outlined p-4 !text-[12em] opacity-50  text-gray-300",
                        "restart_alt"
                    }
                },
                //Card description
                div {
                    class: "h-24 p-4",
                    h1{
                        class:"text-xl text-left font-bold",
                        "Headless UI components"
                    },
                    p {
                        class: "text-lg text-left break-words",
                        "Simple unstyled, open-source, accessible UI components, to use in your applications "
                    }
                }
            }
        }
    }
}
