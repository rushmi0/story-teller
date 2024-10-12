#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::account_card_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_2.jpg"));

#[component]
pub fn AccountCard() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "account-card",

            img { class: "profile-image",
                src: _IMG,
                alt: "user",
            }

            div { class: "user-info",
                h3 { class: "user-name", "Ricky Park" }
                h6 { class: "user-location", "New York" }
                p { class: "user-description", "User interface designer and " br {} "front-end developer" }
            }

        }
    }
}
