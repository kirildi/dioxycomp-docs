//! Run with:
//!
//! ```sh
//! dx build --features web --release
//! dx serve
//! ```

#![allow(unused)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub mod app;
pub mod pages;
pub mod router;
use router::PageRouter::Route;

fn main() {
    dioxus::launch(app::App);
}
