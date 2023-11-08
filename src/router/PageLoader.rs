#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::headless::components::ButtonPage::ButtonPage;
use crate::pages::headless::components::CheckboxPage::CheckboxPage;
use crate::pages::headless::components::RadioPage::RadioPage;
use crate::pages::headless::components::SelectPage::SelectPage;

#[inline_props]
pub fn PageLoader(cx: Scope, name: String) -> Element {
    match name.as_str() {
        "Button" => cx.render(rsx! {
            ButtonPage { name: String::from("Button")}
        }),
        "Checkbox" => cx.render(rsx! {
            CheckboxPage { name: String::from("Checkbox")}
        }),
        "Radio" => cx.render(rsx! {
            RadioPage { name: String::from("Radio")}
        }),
        "Select" => cx.render(rsx! {
            SelectPage { name: String::from("Select")}
        }),
        _ => cx.render(rsx! {
            p { "no page to render" }
        }),
    }
}
