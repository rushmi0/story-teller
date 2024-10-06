#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{
    Banner,
    SearchBar,
    CheckBox,
    Story
};

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Banner {}
        SearchBar {}
        div {
            CheckBox {}
            Story {}
        }
    }
}