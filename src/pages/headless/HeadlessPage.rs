#![allow(non_snake_case)]

use dioxus::prelude::*;

use super::components::SideBarNav::SideBarNav;
use crate::router::PageRouter::Route;

#[component]
pub fn HeadlessPage() -> Element {
    rsx! {
        section {
            class: "flex min-h-dvh pt-0",
            SideBarNav {},
            main {
                class: "grow relative px-24 pt-12 pb-6 text-xl lg:w-full lg:h-full leading-normal",
                Outlet::<Route> {}
            },
        }
    }
}
