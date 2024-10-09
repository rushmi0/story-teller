#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{
    Banner,
    CheckBox,
    SearchBar,
    Story
};
use crate::styles::grid_style::STYLE;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        style { {STYLE} }
        Banner {}
        SearchBar {}
        div { class: "control-box",
            //style: "background-color:yellow;",

            div { class: "col-xs-12 col-sm-4 col-lg-5 col-xl-3",
                style: "background-color:red;", // สำหรับ Debug
                CheckBox {}
            }
            div { class: "col-xs-12 col-sm-8 col-lg-8 col-xl-9",
                style: "background-color:green;", // สำหรับ Debug
                Story {}
            }
        }

    }
}