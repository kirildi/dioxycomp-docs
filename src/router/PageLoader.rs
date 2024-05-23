#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::headless::components::ButtonPage::ButtonPage;
use crate::pages::headless::components::CheckboxPage::CheckboxPage;
use crate::pages::headless::components::RadioPage::RadioPage;
use crate::pages::headless::components::SelectPage::SelectPage;

#[component]
pub fn PageLoader(name: String) -> Element {
    match name.as_str() {
        "Button" => rsx! { ButtonPage { name: String::from("Button")} },
        "Checkbox" => rsx! { CheckboxPage { name: String::from("Checkbox")} },
        "Radio" => rsx! { RadioPage { name: String::from("Radio")} },
        "Select" => rsx! { SelectPage { name: String::from("Select")} },
        _ => rsx! { p { "no page to render" } },
    }
}
