#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use web_sys::window;
use crate::components::search_bar::SearchBar;
use crate::styles::checkbox_style::STYLE;

// กำหนดค่า constant สำหรับไอคอน
const _ICON_FILTER: &str = manganis::mg!(file("src/assets/filter-icon.svg"));
const _ICON_ARROW: &str = manganis::mg!(file("src/assets/chevron-down.svg"));


#[component]
pub fn CheckBox() -> Element {

    // ใช้ use_signal เพื่อเก็บรายการของ key words ที่ผู้ใช้เลือก
    let mut key_word_list = use_signal(Vec::<String>::new);

    let mut is_dropdown = use_signal(|| false);

    //let mut window_size = use_signal(|| (0, 0));

    // สร้าง state เพื่อเก็บขนาดหน้าจอ
    let window = window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;


    // Debug แสดงข้อมูลของ key_word_list ปัจจุบันในตอนที่ component ถูก render
    info!("Current key word list: {:?}", key_word_list.read());
    info!("Screen size: {width} x {height}");

    rsx! {
        // ใส่ style สำหรับ component
        style { {STYLE} }

        // เรียกใช้ SearchBar component
        SearchBar {}
        div { class: "checkbox-container col-xs-12 col-sm-4 col-lg-1",
            div { class: "checkbox-sidebar col-xs-11",

                // ปุ่มกดเพื่อเปิด/ปิด dropdown
                h3 { style: "color: white", "Current Screen Size: Width:{width} x Height{height}" }
                div {
                    button {
                        class: "icon-container",
                        onclick: move |_| {
                            let mut dropdown = is_dropdown.write();
                            *dropdown = !*dropdown; // สลับสถานะ dropdown
                        },
                        img { src: "{_ICON_FILTER}" }
                        img { class: "col-lg-hidden col-ms-hidden", src: "{_ICON_ARROW}" }
                    }
                }

                // เนื้อหาที่จะถูกซ่อนสำหรับจอมือถือ
                if *is_dropdown.read() {
                    div { class: "checkbox-pt",
                        h3 { class: "header", "General" }
                        ul { class: "detail ",
                            // Checkbox สำหรับตัวเลือก "Chill"
                            li {
                                input {
                                    class: "filter-checkbox",
                                    r#type: "checkbox",
                                    id: "chill",
                                    // เมื่อคลิกที่ checkbox จะเรียก closure นี้
                                    onclick: move |_| {
                                        // เขียน (write) ค่าลงใน key_word_list
                                        let mut list = key_word_list.write();
                                        // ถ้าคำว่า "Chill" มีอยู่ใน list ให้ลบออก
                                        if list.contains(&"Chill".to_string()) {
                                            list.retain(|x| x != "Chill");
                                        } else {
                                            // ถ้ายังไม่มีให้เพิ่มเข้าไปใน list
                                            list.push("Chill".to_string());
                                        }
                                        // Debug การคลิก checkbox ของ "Chill"
                                        info!("Chill checkbox clicked");
                                    }
                                }
                                label { class: "filter-label", "Chill" }
                            }


                            // Checkbox สำหรับตัวเลือก "Dramatic"
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


                            // Checkbox สำหรับตัวเลือก "Happy"
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


                            // Checkbox สำหรับตัวเลือก "Sad"
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


                            // Checkbox สำหรับตัวเลือก "Hopeful"
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


                            // Checkbox สำหรับตัวเลือก "Fantasy"
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


                            // Checkbox สำหรับตัวเลือก "Romantic"
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


                            // Checkbox สำหรับตัวเลือก "Relaxing"
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
}