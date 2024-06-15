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

use fs_extra::dir::copy;

// Generate all routes and output them to the docs path
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let index_html = std::fs::read_to_string("docs/index.html").unwrap();
    let main_tag = r#"<div id="main">"#;
    let (before_body, after_body) = index_html.split_once(main_tag).expect("main id not found");
    let after_body = after_body
        .split_once("</div>")
        .expect("main id not found")
        .1;
    let wrapper = DefaultRenderer {
        before_body: before_body.to_string() + main_tag,
        after_body: "</div>".to_string() + after_body,
    };
    let mut renderer = IncrementalRenderer::builder()
        .static_dir("docs_static")
        .map_path(|route| {
            let mut path = std::env::current_dir().unwrap();
            path.push("docs_static");
            for segment in route.split('/') {
                path.push(segment);
            }
            path
        })
        .build();
    renderer.renderer_mut().pre_render = true;
    pre_cache_static_routes::<Route, _>(&mut renderer, &wrapper)
        .await
        .unwrap();

    // Copy everything from docs_static to docs
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    options.copy_inside = true;
    std::fs::create_dir_all(format!("./docs")).unwrap();
    fs_extra::dir::move_dir("./docs_static", &format!("./docs"), &options).unwrap();
}

// Hydrate the page
#[cfg(not(feature = "server"))]
fn main() {
    #[cfg(all(feature = "web", not(feature = "server")))]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
        // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        dioxus_web::launch::launch(
            app::App,
            Default::default(),
            dioxus_web::Config::default().hydrate(true),
        )
    }
}
