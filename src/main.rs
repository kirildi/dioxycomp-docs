//! Run with:
//!
//! ```sh
//! dx build --features web --release
//! cargo run --features ssr
//! ```

#![allow(unused)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod app;
pub mod pages;
pub mod router;
use router::PageRouter::Route;

// Hydrate the page
// #[cfg(not(feature = "server"))]
fn main() {
    #[server(endpoint = "static_routes", output = server_fn::codec::Json)]
    async fn static_routes() -> Result<Vec<String>, ServerFnError> {
        // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
        Ok(Route::static_routes()
            .iter()
            .map(ToString::to_string)
            .collect())
    }
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    dioxus::server::IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(app::App);
}
