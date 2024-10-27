#![allow(non_snake_case)]

use std::thread::Scope;
use dioxus::prelude::*;

#[component]
pub fn NewStory() -> Element {
    rsx! {
        h1 { "New Story" }
    }
}
