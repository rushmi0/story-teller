#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::components::account::auth_card::AuthCard;
use crate::components::account::state_show::StateShow;
use crate::styles::banner_style::STYLE;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));

/// คอมโพเนนต์ Banner รับ app_state เป็น StateShow
/// และใช้เพื่อแสดงปุ่ม Login และ AuthCard
#[component]
pub fn Banner(app_state: StateShow) -> Element {
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
                        // โดยใช้ app_state ที่ส่งมาจาก HomePage
                        let mut is_dropdown: Write<bool> = app_state.show_auth_card.write();
                        *is_dropdown = !*is_dropdown;
                        info!("Login clicked");
                    },
                    "Login"
                }
            }

            // หาก show_auth_card เป็น true จะทำการแสดง AuthCard
            if *app_state.show_auth_card.read() {
                // ส่ง app_state ให้ AuthCard
                AuthCard { app_state: app_state.clone() }
            }
        }
    }
}