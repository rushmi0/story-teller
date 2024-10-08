#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{
    Banner,
    CheckBox,
    SearchBar,
    Story
};

#[component]
pub fn HomePage() -> Element {
    rsx! {

        Banner {}
        SearchBar {}

        div { class: "col-xs-12 col-ms-6 col-lg-2",
            //style: "background-color:red;", // สำหรับ Debug
            CheckBox {}
        }
        div { class: "col-xs-12 col-ms-6 col-lg-8 ",
            //style: "background-color:green; ", // สำหรับ Debug
            Story {}
        }

    }
}