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
use serde::{Deserialize, Serialize};

use dioxycomp_headless::components::*;

pub mod app;
pub mod pages;
pub mod router;

use router::PageRouter::Route;
// Generate all routes and output them to the docs path
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let wrapper = DefaultRenderer {
        before_body: r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width,
        initial-scale=1.0">
        <title>Dioxus Application</title>
    </head>
    <body>"#
            .to_string(),
        after_body: r#"</body>
    </html>"#
            .to_string(),
    };
    let mut renderer = IncrementalRenderer::builder().build();
    pre_cache_static_routes::<Route, _>(&mut renderer, &wrapper)
        .await
        .unwrap();
}
// async fn main() {
//     pre_cache_static_routes_with_props(
//         &ServeConfigBuilder::new_with_router(dioxus_fullstack::router::FullstackRouterConfig::<
//             Route,
//         >::default())
//         .assets_path("docs")
//         .incremental(IncrementalRendererConfig::default().static_dir("docs"))
//         .build(),
//     )
//     .await
//     .unwrap();
// }

// Hydrate the page
#[cfg(not(feature = "server"))]
fn main() {
    #[cfg(all(feature = "web", not(feature = "server")))]
    dioxus_web::launch::launch(
        app::App,
        Default::default(),
        dioxus_web::Config::new().hydrate(true),
    )
}
