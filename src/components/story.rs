#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::story_style::STYLE;

#[component]
pub fn Story() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "col-xs-12 col-sm-12 col-lg-10",

        }
    }
}