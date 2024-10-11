#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::{
    Banner,
    CheckBox,
    SearchBar,
    Story
};
use crate::components::account::state_show::StateShow;
use crate::styles::grid_style::STYLE;


/// คอมโพเนนต์ HomePage สร้าง instance ของ StateShow
/// เพื่อใช้ร่วมกับ Banner และส่วนอื่นๆ
#[component]
pub fn HomePage() -> Element {
    // สร้าง instance ของ StateShow เพื่อจัดการสถานะการแสดง AuthCard
    let app_state = StateShow::new();

    rsx! {
        style { {STYLE} }

        // ส่ง app_state ไปยัง Banner
        Banner { app_state: app_state }

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