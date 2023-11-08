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
                class: "ml-0 order-first"
                //TODO Convert to LOGO component
            },
            Nav {},
        },

    })
}
