#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use crate::components::account::{AuthCard, AccountCard};
use crate::styles::banner_style::STYLE;

use web_sys::console;
use nostr_sdk::prelude::*;

use crate::components::shared::{
    SharedAccountVisibility,
    SharedAuthVisibility
};
use crate::model::local_storage::LocalStorage;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));
const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_2.jpg"));

#[component]
pub fn Banner(state_auth: SharedAuthVisibility, state_account: SharedAccountVisibility) -> Element {
    // สร้าง Signal เพื่อควบคุมการแสดง AccountCard
    let mut show_account_card = use_signal(|| false);

    // สร้าง Signal เพื่อเก็บ key จาก LocalStorage
    let story_teller_keys = use_signal(|| Vec::new());

    // สร้าง Signal เพื่อเก็บข้อมูลที่ดึงมาจาก LocalStorage
    let story_teller_data = use_signal(|| String::new());

    // ตรวจสอบ Local Storage
    use_effect({
        let mut story_teller_keys = story_teller_keys.clone();
        move || {
            // ใช้ LocalStorage ในการตรวจสอบค่า
            if let Some(storage) = LocalStorage::get_all_keys() {
                // เก็บเฉพาะ key ที่ขึ้นต้นด้วย 'story-teller_' ลงใน Signal
                let filtered_keys: Vec<String> = storage
                    .into_iter()
                    .filter(|key| key.starts_with("story-teller_"))
                    .collect();

                if !filtered_keys.is_empty() {
                    story_teller_keys.set(filtered_keys);
                    state_account.show_account.set(true);
                }
            }
            ()
        }
    });

    // ตรวจสอบว่า show_account ถูกตั้งค่าหรือไม่
    use_effect({
        let story_teller_keys = story_teller_keys.clone();
        let mut story_teller_data = story_teller_data.clone();
        move || {
            if !story_teller_keys.is_empty() {
                // ดึงค่าคีย์คำแหน่งแรก
                if let Some(first_key) = story_teller_keys.get(0) {
                    // ดึงข้อมูลจาก LocalStorage
                    if let Some(data) = LocalStorage::get(&first_key) {
                        story_teller_data.set(data);
                    }
                }
            }
            ()
        }
    });


    // ใช้ use_effect เพื่อ log ข้อมูลเมื่อ show_account เป็น true
    use_effect({
        let story_teller_data = story_teller_data.clone();
        move || {
            if *state_account.show_account.read() {
                console::log_1(&format!("Story Teller Data: {}", *story_teller_data.read()).into());
            }
            ()
        }
    });


    rsx! {
        style { {STYLE} }
        div { class: "item-nav", id: "nav",
            img {
                src: "{IMG_BANNER}"
            }

            if *state_account.show_account.read() {

                // แสดงรูป Profile หากมีข้อมูลใน Local storage
                div { class: "nav-profile-round",
                    button {
                        onclick: move |_| {
                            let mut is_show_account = show_account_card.write();
                            *is_show_account = !*is_show_account;
                            info!("Profile clicked");
                        },
                        img { src: "{_IMG}" }
                    }
                }

                if *show_account_card.read() {
                    AccountCard {
                        //data: story_teller_data.clone()
                    }
                }

            } else {
                // แสดงปุ่ม Login หากไม่มีข้อมูลใน Local storage
                div { class: "col-xs-3 col-sm-3 col-md-2 col-lg-1 col-xl-1",
                    button { class: "nav-login login-position",
                        onclick: move |_| {
                            let mut is_dropdown = state_auth.show_auth_card.write();
                            *is_dropdown = !*is_dropdown;
                            info!("Login clicked");
                        },
                        "Login"
                    }
                }

                if *state_auth.show_auth_card.read() {
                    // หน้า UI ตัวเลือกสำหรับ Login
                    AuthCard {
                        state_auth: state_auth.clone(),
                        state_account: state_account.clone()
                    }
                }
            }
        }
    }
}
