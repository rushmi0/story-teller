#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use web_sys::window;
use crate::components::account::{AuthCard, AccountCard};
use crate::styles::banner_style::STYLE;

use crate::components::shared::{
    SharedAccountVisibility,
    SharedAuthVisibility
};

const IMG_BANNER: &str = manganis::mg!(file("src/assets/nav-icon.svg"));
const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_2.jpg"));

#[component]
pub fn Banner(state_auth: SharedAuthVisibility, state_account: SharedAccountVisibility) -> Element {

    // ตรวจสอบ Local storage เมื่อ component ถูก mount
    use_effect({
        move || {
            if let Some(storage) = window().and_then(|win| win.local_storage().ok().flatten()) {
                // ค้นหา key ที่ขึ้นต้นด้วย 'story-teller_'
                for i in 0..storage.length().unwrap_or(0) {
                    if let Some(key) = storage.key(i).ok().flatten() {
                        if key.starts_with("story-teller_") {
                            state_account.show_account.set(true);
                            break;
                        }
                    }
                }
            }
            ()
        }
    });

    let mut show_account_card: Signal<bool> = use_signal(|| false);

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
                    AccountCard {}
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
                    //AccountCard {}
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
