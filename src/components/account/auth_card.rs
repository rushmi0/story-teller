#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::account::state_show::StateShow;
use crate::styles::auth_card_style::STYLE;

const _ICON: &str = manganis::mg!(file("src/assets/icon.svg"));
const _CROSS: &str = manganis::mg!(file("src/assets/fi-rr-cross-small.svg"));

#[component]
pub fn AuthCard(app_state: StateShow) -> Element {

    // ฟังก์ชันจัดการการคลิกที่ overlay เพื่อซ่อน AuthCard
    let handle_click_overlay = move |_| {
        // ตั้งค่าให้ show_auth_card เป็น false หรือก็คือซ่อน AuthCard
        app_state.show_auth_card.set(false);
    };

    // ฟังก์ชันจัดการการคลิกที่ปุ่ม cross เพื่อปิด Card
    let handle_click_cross = move |_| {
        // ซ่อน AuthCard เมื่อคลิกที่ปุ่ม cross
        app_state.show_auth_card.set(false);
    };

    rsx! {
        style { {STYLE} }

        // เบลอพื้นหลัง
        div { id: "overlay",
            onclick: handle_click_overlay
        }

        div { id: "form-ui",
            div { id: "form-card",

                // เพิ่มไอคอน cross ที่มุมขวาบน
                div { id: "close-button",
                    onclick: handle_click_cross,
                    img { src: "{_CROSS}" }
                }

                form { id: "form",
                    div { id: "form-body",

                        div { id: "welcome-lines",

                            div { id: "logo-top-card",
                                img { src: "{_ICON}" }
                            }
                            div { id: "welcome-line-2", "Sign in to Story Teller" }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button", r#type: "submit", "Sign in with extension" }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button", r#type: "submit", "Sign in with nsec" }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button", r#type: "submit", "Sign up" }
                        }
                    }
                }
            }
        }
    }
}
