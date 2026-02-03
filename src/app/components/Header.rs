#![allow(non_snake_case)]
use dioxus::prelude::*;

use super::Nav::Nav;
use crate::router::PageRouter::Route;

#[component]
pub fn Header() -> Element {
    static HEADER_LOGO: Asset = asset!("/public/assets/image/dioxy_logo_v0.1.png");

    rsx! {
        header{
            class: "flex sticky w-full top-0 h-24 justify-center bg-neutral-800 z-20",
            div {
                class: "mr-auto h-24",
                img {
                    class: "w-24 h-24 p-4",
                    src: { HEADER_LOGO },
                    alt: "Dioxycomp Logo"
                }
            },
            Nav {},
        },
    }
}
