#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::banner_style::STYLE;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));

#[component]
pub fn Banner() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "item-center", id: "nav",
            img {
                src: "{IMG_BANNER}"
            }
        }
    }
}
