#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{Banner, SearchBar};
use crate::styles::layout_style::STYLE;

#[component]
pub fn ArticlePage() -> Element {
    rsx! {
        style { {STYLE} }
        //Banner {}
        SearchBar {}

        div { class: "control-box",

        }
    }
}
