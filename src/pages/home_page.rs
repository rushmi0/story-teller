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
        div { class: "col-xs-12 col-lg-12 col-sm-12",
            style: "display: flex;",

            div { class: "col-lg-3 col-sm-3 col-xs-12",
                style: "background-color:red;",
                CheckBox {}
            }

            div { class: "col-lg-10 col-sm-8 col-xs-12",
                style: "background-color:green;",
                Story {}
            }
        }
    }
}
