#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use js_sys::wasm_bindgen::JsValue;
use crate::components::account::{AuthCard, AccountCard};
use crate::styles::banner_style::STYLE;

use web_sys::console;
use nostr_sdk::{
    //Event,
    serde_json
};

use crate::components::shared::{SharedAccountVisibility, SharedAuthVisibility, SharedMetadataVisibility};
use crate::model::local_storage::LocalStorage;

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));
const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_2.jpg"));

#[component]
pub fn Banner(
    state_auth: SharedAuthVisibility,
    state_account: SharedAccountVisibility,
    state_metadata: SharedMetadataVisibility
) -> Element {

    // สร้าง Signal เพื่อควบคุมการแสดง AccountCard
    let mut show_account_card = use_signal(|| false);

    // เก็บ key จาก LocalStorage
    let story_teller_keys = use_signal(|| Vec::new());

    // ตรวจสอบ Local Storage
    use_effect({
        let mut story_teller_keys = story_teller_keys.clone();
        move || {

            // ดึงค่า keyทั้งหมดใน LocalStorage
            if let Some(storage) = LocalStorage::get_all_keys() {

                // เก็บเฉพาะ key ที่ขึ้นต้นด้วย 'story-teller_' ลงใน `story_teller_keys`
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
        move || {
            if !story_teller_keys.is_empty() {
                // ดึงค่าคีย์คำแหน่งแรก
                if let Some(first_key) = story_teller_keys.get(0) {

                    // ดึงข้อมูลจาก LocalStorage
                    if let Some(data) = LocalStorage::get(&first_key) {

                        state_metadata.raw_metadata.set(data.to_string().clone());
                        state_metadata.metadata.set(serde_json::from_str(&data).unwrap());

                        if let Some(event) = state_metadata.metadata.read().as_ref() {
                            // ดึง content จาก event
                            let metadata_content = &event.content;
                            //console::log_1(&JsValue::from_str(metadata_content));

                            state_metadata.user_metadata.set(serde_json::from_str(metadata_content).unwrap());


                        } else {
                            console::log_1(&JsValue::from_str("No event metadata available."));
                        }

                    }
                }
            }
            ()
        }
    });


    // ใช้ use_effect เพื่อ log ข้อมูลเมื่อ show_account เป็น true
    use_effect({
        let state_metadata = state_metadata.clone();
        move || {
            if *state_account.show_account.read() {

                // สร้าง Event object จาก JSON string
                // let data_object: Event = serde_json::from_str(&data_string).unwrap();
                // let content = serde_json::from_str(&data_object.content).unwrap();

                // อ่านค่าจาก raw_metadata
                let metadata = state_metadata.raw_metadata.read();
                console::log_1(&JsValue::from_str(&metadata));
            }

            ()
        }
    });

    // ดึงค่าจาก state_metadata.user_metadata
    let profile_image = state_metadata.user_metadata.read().picture.clone();

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
                        img { src: "{profile_image}" }
                    }
                }

                if *show_account_card.read() {
                    AccountCard { state_metadata: state_metadata }
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
