#![allow(non_snake_case)]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::components::Main::Main;
use crate::pages::home::HomePage::HomePage;
use crate::router::PageLoader::PageLoader;

use crate::pages::headless::HeadlessPage::HeadlessPage;

#[derive(Clone, Routable, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Main)]
        #[route("/")]
        HomePage {},
        #[nest("/headless")]
            #[layout(HeadlessPage)]                
                #[route("/:name")]
                PageLoader { name: String },
            #[end_layout]
        #[end_nest]
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
