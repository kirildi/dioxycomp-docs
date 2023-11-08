//! Run with:
//!
//! ```sh
//! dx build --features web --release
//! cargo run --features ssr
//! ```

#![allow(unused)]
use dioxus::prelude::*;
use dioxus_fullstack::{launch, prelude::*};
use dioxus_router::prelude::*;
use dioxycomp_headless::components::*;

pub mod app;
pub mod pages;
pub mod router;

use router::PageRouter::Route;

// Generate all routes and output them to the docs path
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    pre_cache_static_routes_with_props(
        &ServeConfigBuilder::new_with_router(dioxus_fullstack::router::FullstackRouterConfig::<
            Route,
        >::default())
        .assets_path("docs")
        .incremental(IncrementalRendererConfig::default().static_dir("docs"))
        .build(),
    )
    .await
    .unwrap();
}

// Hydrate the page
#[cfg(feature = "web")]
fn main() {
    dioxus_web::launch_with_props(
        dioxus_fullstack::router::RouteWithCfg::<Route>,
        dioxus_fullstack::prelude::get_root_props_from_document()
            .expect("Failed to get root props from document"),
        dioxus_web::Config::default().hydrate(true),
    );
}

#[cfg(not(any(feature = "web", feature = "ssr")))]
fn main() {}
