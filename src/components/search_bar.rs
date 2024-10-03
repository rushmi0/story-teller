#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::search_bar_style::STYLE;

#[component]
pub fn SearchBar() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "search-bar",
            input { r#type: "text", placeholder: "Search Music or Backsound" }
        }
    }
}
