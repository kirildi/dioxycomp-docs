#![allow(non_snake_case)]
use super::components::HeadlessCard::HeadlessCard;
use dioxus::prelude::*;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "h-56 mx-auto pt-12 bg-gray-800",
            h1 {
                class: "h-16 text-3xl lg:text-5xl text-center leading-normal font-bold",
                "DIOXYCOMP"
            },
            h2 {
                class: "text-2xl lg:text-3xl text-center leading-loose",
                "An UI component library for the ",
                a {
                    class: "text-blue-300",
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    "Dioxus framework"
                }
            }
        },
        section {
            class: "p-4 bg-gray-800",
            HeadlessCard {}
        }
    })
}
