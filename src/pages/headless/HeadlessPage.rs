#![allow(non_snake_case)]

use dioxus::prelude::*;

use super::components::SideBarNav::SideBarNav;
use crate::router::PageRouter::Route;

#[component]
pub fn HeadlessPage() -> Element {
    rsx! {
        div {
            class: "flex",
            SideBarNav {},
            main {
                class: "grow relative px-24 pt-12 pb-6 text-xl bg-zinc-900 lg:ml-72 lg:w-full lg:h-full leading-normal",
                Outlet::<Route> {}
            },
        }
    }
}
