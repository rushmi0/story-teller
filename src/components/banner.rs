#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::components::account::auth_card::AuthCard;
use crate::components::shared::SharedAuthVisibility;
use crate::styles::banner_style::STYLE;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));

#[component]
pub fn Banner(state_channel: SharedAuthVisibility) -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "item-nav", id: "nav",
            img {
                src: "{IMG_BANNER}"
            }

            div { class: "col-xs-3 col-sm-3 col-md-2 col-lg-1 col-xl-1",
                button { class: "nav-login login-position",
                    onclick: move |_| {
                        // เมื่อคลิกที่ปุ่ม Login จะทำการเปลี่ยนสถานะของ show_auth_card
                        // โดยใช้ state_channel ที่ส่งมาจาก HomePage
                        let mut is_dropdown: Write<bool> = state_channel.show_auth_card.write();
                        *is_dropdown = !*is_dropdown;
                        info!("Login clicked");
                    },
                    "Login"
                }
            }

            // หาก show_auth_card เป็น true จะทำการแสดง AuthCard
            if *state_channel.show_auth_card.read() {
                // ส่ง state_channel ให้ AuthCard
                AuthCard { state_channel: state_channel.clone() }
            }
        }
    }
}