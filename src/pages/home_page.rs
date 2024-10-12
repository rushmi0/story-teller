#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{
    Banner,
    CheckBox,
    SearchBar,
    Story
};

use crate::components::shared::SharedAuthVisibility;
use crate::styles::layout_style::STYLE;

#[component]
pub fn HomePage() -> Element {

    let state_channel = SharedAuthVisibility::new();

    rsx! {
        style { {STYLE} }

        Banner { state_channel: state_channel }

        SearchBar {}
        div { class: "control-box",
            //style: "background-color:yellow;",

            div { class: "col-xs-12 col-sm-4 col-md-4 col-lg-3 col-xl-2",
                //style: "background-color:red;", // สำหรับ Debug
                CheckBox {}
            }
            div { class: "col-xs-12 col-sm-8 col-md-8 col-lg-9 col-xl-10",
                //style: "background-color:green;", // สำหรับ Debug
                Story {}
            }
        }

    }
}