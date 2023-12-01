#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::Nav::Nav;
use crate::router::PageRouter::Route;

#[inline_props]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        header{
            class: "flex sticky w-full top-0 h-24 justify-between bg-neutral-800 z-20",
            div {
                class: "ml-0 h-24 order-first",
                img {
                    class: "w-24 h-24 p-4",
                    src: "/dioxycomp-docs/assets/image/dioxy_logo_v0.1.svg",
                    alt: "Dioxycomp Logo"
                }
            },
            Nav {},
        },

    })
}
