#![allow(non_snake_case)]

use std::str::FromStr;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use nostr_sdk::Kind;
use nostr_sdk::nips::nip07;
use crate::components::content::article::markdown_to_html;
use crate::nostr::NostrClient;
use nostr_sdk::prelude::{
    Nip07Signer,
    UnsignedEvent,
    PublicKey,
    Timestamp,
    EventId
};
use crate::styles::new_story_style::STYLE;

const _PLUS: &str = manganis::mg!(file("src/assets/plus.svg"));
const _EDIT_TEXT: &str = manganis::mg!(file("src/assets/text.svg"));
const _PREVIEW: &str = manganis::mg!(file("src/assets/preview.svg"));

#[component]
pub fn NewStory() -> Element {
    // สร้างตัวแปร use_signal สำหรับเก็บข้อมูลจาก input
    let mut input_text = use_signal(|| None::<String>);

    // สร้างตัวแปร use_signal สำหรับเก็บสถานะการแสดง textarea
    let mut show_textarea = use_signal(|| false);

    // สร้างตัวแปรเพื่อควบคุมการทำงานของ use_future เมื่อกดปุ่ม Submit
    let mut submit_trigger = use_signal(|| false);


    if *submit_trigger.read() {
        // Logic สำหรับการทำงานหลังจากกด Submit โดยใช้ use_future
        use_future(move || {
            let input_text = input_text.clone();
            let mut submit_trigger = submit_trigger.clone();

            async move {

                let content = input_text.read().as_deref().unwrap_or("").to_string();
                if content.is_empty() {
                    error!("Content is empty. Submission aborted.");
                    return;
                }

                let client = NostrClient::setup_and_connect().await
                    .expect("Failed to setup client");

                match Nip07Signer::new() {
                    Ok(signer) => {
                        let pubkey_str = signer.get_public_key().await.unwrap();
                        let pubkey = PublicKey::from_str(&pubkey_str.to_hex()).unwrap();

                        let created_at = Timestamp::now(); // กำหนดเวลา Unix time ปัจจุบัน
                        let kind = Kind::LongFormTextNote;
                        let tags = vec![]; // กำหนด tags เป็น vector ว่าง

                        let unsigned_event = UnsignedEvent {
                            id: None, // `id` จะถูกสร้างตอนที่เซ็น event
                            pubkey,
                            created_at,
                            kind,
                            tags,
                            content: content.clone(),
                        };

                        info!("Unsigned Event: {:?}", unsigned_event);

                        match signer.sign_event(unsigned_event).await {
                            Ok(signed_event) => {
                                info!("Event signed successfully: {:?}", signed_event);

                                client.send_event(signed_event).await
                                    .expect("TODO: panic message");

                                // รีเซ็ตค่า submit_trigger กลับเป็น false
                                submit_trigger.set(false);
                            }
                            Err(e) => error!("Failed to sign event: {:?}", e),
                        }
                    }
                    Err(e) => error!("Error initializing Nip07Signer: {:?}", e),
                }
            }
        });
    }



    rsx! {
        style { {STYLE} }
        div { class: "story-write-box",
            div { class: "write-box",
                div { class: "title-box",
                    h2 { "Explanation" }
                    div { class: "action-btn",
                        button { class: "submit-btn",
                            r#type: "button",
                            onclick: move |_| {
                                if let Some(ref text) = *input_text.read() {
                                    info!("Input Text Submitted: {}", text);
                                    // กำหนดค่าให้ submit_trigger เพื่อให้ use_future ทำงาน
                                    submit_trigger.set(true);
                                }
                            },
                            span { "Submit" }
                        }
                        button { class: "cancel-btn",
                            r#type: "button",
                            onclick: move |_| {
                                // ล้างข้อมูลใน textarea
                                input_text.set(None);
                            },
                            span { "Cancel" }
                        }
                    }
                }

                div { class: "option-btn",
                    button { class: "option-btn-item",
                        r#type: "button",
                        onclick: move |_| show_textarea.set(true),
                        img { src: "{_EDIT_TEXT}" },
                        span { "Edit" }
                    }
                    button { class: "option-btn-item",
                        r#type: "button",
                        onclick: move |_| show_textarea.set(false),
                        img { src: "{_PREVIEW}" },
                        span { "Preview" }
                    }
                }

                // แสดง textarea เฉพาะเมื่อ show_textarea เป็น true
                if *show_textarea.read() {
                    textarea {
                        class: "input-long-text",
                        placeholder: "Type...",
                        rows: "10",
                        // ใช้ค่าใน input_text เป็นค่าเริ่มต้นของ textarea
                        value: input_text.read().as_deref().unwrap_or(""),
                        oninput: move |evt| {
                            let value = evt.value().clone();
                            input_text.set(Some(value));
                        }
                    }
                } else {
                    // แสดงผลข้อความในโหมด Preview โดยดึงข้อมูลจาก input_text
                    div { class: "markdown-field-text",
                        dangerous_inner_html: markdown_to_html(input_text.read().as_deref().unwrap_or(""))
                    }
                }
            }
        }
    }
}
