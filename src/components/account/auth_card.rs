use dioxus::prelude::*;
use nostr_sdk::nips::nip07;
use nostr_sdk::ToBech32;
use web_sys::console;
use crate::components::shared::SharedAuthVisibility;
use crate::styles::auth_card_style::STYLE;

const _ICON: &str = manganis::mg!(file("src/assets/icon.svg"));
const _CROSS: &str = manganis::mg!(file("src/assets/fi-rr-cross-small.svg"));

#[component]
pub fn AuthCard(state_channel: SharedAuthVisibility) -> Element {

    // ฟังก์ชันจัดการการคลิกที่ overlay เพื่อซ่อน AuthCard
    let handle_click_overlay = move |_| {
        state_channel.show_auth_card.set(false);
    };

    // ฟังก์ชันจัดการการคลิกที่ปุ่ม cross เพื่อปิด Card
    let handle_click_cross = move |_| {
        state_channel.show_auth_card.set(false);
    };

    // ฟังก์ชันจัดการการคลิกที่ปุ่ม "Sign in with extension"
    let handle_sign_in_with_extension = move |_| {
        console::log_1(&"Sign in with extension clicked!".into());
        // เรียกใช้ use_future เพื่อดำเนินการ async
        use_future({
            // เก็บ state_channel เพื่อใช้ใน future
            let mut state_channel = state_channel.clone();
            move || async move {
                match nip07::Nip07Signer::new() {
                    Ok(signer) => {
                        match signer.get_public_key().await {
                            Ok(public_key) => {

                                let npub = public_key.to_bech32().unwrap();
                                console::log_1(&npub.into());

                                // ปิด AuthCard หลังจากได้รับ public_key สำเร็จ
                                state_channel.show_auth_card.set(false);
                            }
                            Err(e) => {
                                console::log_1(&format!("Error getting public key: {:?}", e).into());
                            }
                        }
                    }
                    Err(e) => {
                        console::log_1(&format!("Error initializing Nip07Signer: {:?}", e).into());
                    }
                }
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
                }
            }
        }
    }
}
