#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{Banner, CheckBox};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Banner {}
        CheckBox {}
    }
}