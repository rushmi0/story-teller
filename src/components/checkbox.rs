#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;

use crate::components::search_bar::SearchBar;
use crate::styles::checkbox_style::STYLE;

const _ICON_FILTER: &str = manganis::mg!(file("src/assets/filter-icon.svg"));
const _ICON_ARROW: &str = manganis::mg!(file("src/assets/chevron-down.svg"));

#[component]
pub fn CheckBox() -> Element {
    // ใช้ use_signal เพื่อเก็บรายการของ key words
    let mut key_word_list = use_signal(Vec::<String>::new);

    info!("Current key word list: {:?}", key_word_list.read());

    rsx! {
        style { {STYLE} }
        SearchBar {}
        div { class: "checkbox-container col-xs-12 col-sm-3 col-lg-1",
            div { class: "checkbox-sidebar col-xs-11",

                div { class: "icon-container",
                    img { src: "{_ICON_FILTER}" }
                    img { class: "col-lg-hidden", src: "{_ICON_ARROW}" }
                }

                div { class: "checkbox-pt col-xs-hidden",
                    h3 { class: "header", "General" }
                    ul { class: "detail ",
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "chill",
                                // สร้าง closure ใหม่ทุกครั้งที่ checkbox ถูกคลิก
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Chill".to_string()) {
                                        list.retain(|x| x != "Chill");
                                    } else {
                                        list.push("Chill".to_string());
                                    }
                                    info!("Chill checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Chill" }
                        }
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "dramatic",
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Dramatic".to_string()) {
                                        list.retain(|x| x != "Dramatic");
                                    } else {
                                        list.push("Dramatic".to_string());
                                    }
                                    info!("Dramatic checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Dramatic" }
                        }
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "happy",
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Happy".to_string()) {
                                        list.retain(|x| x != "Happy");
                                    } else {
                                        list.push("Happy".to_string());
                                    }
                                    info!("Happy checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Happy" }
                        }
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "sad",
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Sad".to_string()) {
                                        list.retain(|x| x != "Sad");
                                    } else {
                                        list.push("Sad".to_string());
                                    }
                                    info!("Sad checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Sad" }
                        }
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "hopeful",
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Hopeful".to_string()) {
                                        list.retain(|x| x != "Hopeful");
                                    } else {
                                        list.push("Hopeful".to_string());
                                    }
                                    info!("Hopeful checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Hopeful" }
                        }
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "fantasy",
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Fantasy".to_string()) {
                                        list.retain(|x| x != "Fantasy");
                                    } else {
                                        list.push("Fantasy".to_string());
                                    }
                                    info!("Fantasy checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Fantasy" }
                        }
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "romantic",
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Romantic".to_string()) {
                                        list.retain(|x| x != "Romantic");
                                    } else {
                                        list.push("Romantic".to_string());
                                    }
                                    info!("Romantic checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Romantic" }
                        }
                        li {
                            input {
                                class: "filter-checkbox",
                                r#type: "checkbox",
                                id: "relaxing",
                                onclick: move |_| {
                                    let mut list = key_word_list.write();
                                    if list.contains(&"Relaxing".to_string()) {
                                        list.retain(|x| x != "Relaxing");
                                    } else {
                                        list.push("Relaxing".to_string());
                                    }
                                    info!("Relaxing checkbox clicked");
                                }
                            }
                            label { class: "filter-label", "Relaxing" }
                        }
                    }
                }
            }
        }
    }
}
