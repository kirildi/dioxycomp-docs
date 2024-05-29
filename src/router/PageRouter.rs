#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::components::Main::Main;
use crate::app::App;
use crate::router::PageLoader::PageLoader;

use crate::pages::headless::HeadlessPage::HeadlessPage;

#[derive(Clone, Routable, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(App)]
        #[nest("/headless")]
            #[layout(HeadlessPage)]                
                #[route("/:name")]
                PageLoader { name: String },
            #[end_layout]
        #[end_nest]
        #[route("/")]
        Main {},
        #[route("/:..route")]
        PageNotFound {
            route: Vec<String>,       
        },       
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
    }
}
