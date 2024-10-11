#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::banner_style::STYLE;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));
const IMG_PROFILE: &str = manganis::mg!(file("src/assets/user-add.svg"));

#[component]
pub fn Banner() -> Element {


    rsx! {
        style { {STYLE} }
        div { class: "item-nav", id: "nav",
            img {
                src: "{IMG_BANNER}"
            }


            div { class: "col-xs-3 col-sm-3 col-md-2 col-lg-1 col-xl-1",
                button { class: "nav-login",
                    "Login"
                }
            }
            /*
            img { class: "round",
                src: "{IMG_PROFILE}"
            }
             */

        }
    }
}
