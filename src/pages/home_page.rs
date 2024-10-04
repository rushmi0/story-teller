#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{
    Banner,
    CheckBox,
    Story
};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Banner {}
        CheckBox {}
        Story {}
    }
}