#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use std::time::Duration;
use nostr_sdk::nips::nip07;
use nostr_sdk::{serde_json, Client, EventSource, Filter, Kind};
use web_sys::window;
use crate::components::shared::{SharedAccountVisibility, SharedAuthVisibility};
use crate::styles::auth_card_style::STYLE;

const _ICON: &str = manganis::mg!(file("src/assets/icon.svg"));
const _CROSS: &str = manganis::mg!(file("src/assets/fi-rr-cross-small.svg"));

#[component]
pub fn AuthCard(state_auth: SharedAuthVisibility, state_account: SharedAccountVisibility) -> Element {

    let loading_animation = use_signal(|| false);

    // ฟังก์ชันจัดการการคลิกที่ overlay เพื่อซ่อน AuthCard
    let handle_click_overlay = move |_| {
        state_auth.show_auth_card.set(false);
    };

    // ฟังก์ชันจัดการการคลิกที่ปุ่ม cross เพื่อปิด Card
    let handle_click_cross = move |_| {
        state_auth.show_auth_card.set(false);
    };

    // ฟังก์ชันจัดการการคลิกที่ปุ่ม "Sign in with extension"

    let handle_sign_in_with_extension = move |_| {
        info!("Sign in with extension clicked!");

        // แสดง animation loading ก่อนทำงาน
        use_future({
            // เก็บ state_auth เพื่อใช้ใน future
            let mut state_auth = state_auth.clone();
            let mut loading_animation = loading_animation.clone();
            move || async move {
                match nip07::Nip07Signer::new() {
                    Ok(signer) => {
                        match signer.get_public_key().await {
                            Ok(public_key) => {

                                let client = Client::default();
                                client.add_relay("wss://nos.lol").await.expect("Failed to connect");
                                client.add_relay("wss://relay.damus.io").await.expect("Failed to connect");
                                client.add_relay("wss://relay.notoshi.win").await.expect("Failed to connect");
                                client.add_relay("wss://nostr-01.yakihonne.com").await.expect("Failed to connect");
                                client.connect().await;

                                let filter = Filter::new()
                                    .author(public_key)
                                    .kind(Kind::Metadata);

                                let events = client
                                    .get_events_of(
                                        vec![filter],
                                        EventSource::relays(Some(Duration::from_secs(10))),
                                    )
                                    .await;

                                info!("Events received: {:?}", events);

                                // ตรวจสอบว่ามี events หรือไม่
                                if let Ok(events) = events {
                                    loading_animation.set(true);
                                    if let Some(storage) = window().and_then(|win| win.local_storage().ok().flatten()) {
                                        for event in events {
                                            // แปลง Event เป็น JSON string
                                            let json_string = serde_json::to_string(&event).unwrap();

                                            // บันทึกลง Local Storage
                                            let key = format!("story-teller_{}", &public_key.to_hex());
                                            storage.set_item(&key, &json_string).expect("failed to set item in localStorage");

                                            // กำหนดสถานะปัจจุบันว่ามีการบันทึกข้อมูล Metadata ลง Local Storage
                                            state_account.show_account.set(true);
                                            info!("Stored event with key: {}", key);
                                        }
                                    }
                                }

                                //sleep(Duration::from_secs(2)).await;
                                // ปิด AuthCard หลังจากได้รับ public_key สำเร็จ
                                state_auth.show_auth_card.set(false);
                                if let Some(win) = window() {
                                    win.location().reload().expect("Failed to reload");
                                }

                            }
                            Err(e) => {
                                error!("Error getting public key: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error initializing Nip07Signer: {:?}", e);
                    }
                }

                // ปิด loading animation หลังจากทำงานเสร็จ
                loading_animation.set(false);
            }
        });
    };


    rsx! {
        style { {STYLE} }

        div { id: "overlay",
            onclick: handle_click_overlay
        }

        div { id: "form-ui",
            div { id: "form-card",

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
                            button { id: "submit-button",
                                r#type: "button",
                                onclick: handle_sign_in_with_extension,
                                "Sign in with extension",
                            }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button",
                                r#type: "button",
                                "Sign in with nsec"
                            }
                        }

                        div { id: "submit-button-cvr",
                            button { id: "submit-button",
                                r#type: "button",
                                "Sign up"
                            }
                        }

                    }

                    if *loading_animation.read() {
                        div { class: "lds-ellipsis-container",
                            div { class: "lds-ellipsis",
                               div {} div {} div {}
                            }
                        }
                    }

                }


            }
        }
    }
}
