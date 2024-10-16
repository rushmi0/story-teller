#![allow(non_snake_case)]

use dioxus::prelude::*;
use chrono::{NaiveDateTime, TimeZone, Utc};
use dioxus_logger::tracing::{info, warn};
use reqwest::StatusCode;
use crate::pages::router::Route;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_4.jpg"));
const _MARK: &str = manganis::mg!(file("src/assets/mark.svg"));

#[derive(PartialEq, Props, Clone)]
pub struct StoryCardProps {
    image: String,
    title: String,
    summary: String,
    published_at: String,
    note_id: String,
    author_name: String,
    author_image: String,
}

#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {

    let navigator: Navigator = use_navigator();

    fn format_unix_to_date(unix_timestamp: &str) -> String {
        let timestamp = unix_timestamp.parse::<i64>().unwrap_or(0);
        let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
        let datetime = Utc.from_utc_datetime(&naive);

        // แปลงเป็นรูปแบบ "July 14, 2022"
        datetime.format("%B %d, %Y").to_string()
    }

    // แปลง published_at จาก Unix timestamp เป็นวันที่
    let formatted_date = format_unix_to_date(&props.published_at);

    // ใช้งาน use_resource เพื่อตรวจสอบ URL ของ author_image
    let author_image_future: Resource<Result<String, ()>> = use_resource(move || {
        let value = props.author_image.clone();
        async move {
            let response = reqwest::get(&value).await;

            match response {
                Ok(res) if res.status().is_success() => {
                    //info!("this image: available {}", &value);
                    Ok(value.clone())
                },
                Ok(res) => {
                    // หากได้รับสถานะที่ไม่ใช่ 2xx (เช่น 4xx หรือ 5xx)
                    warn!("this image: unavailable with status: {:?}", res.status());
                    Err(())
                },
                Err(err) => {
                    // หากเกิดข้อผิดพลาดในการเชื่อมต่อ
                    warn!("Failed to fetch image: {:?}", err);
                    Err(())
                },
            }
        }
    });


    let binding = author_image_future.read_unchecked();
    let author_image_url = match &*binding {
        Some(Ok(url)) => url, // ใช้ URL ของ author_image
        _ => &_IMG.to_string(), // ใช้ _IMG แทน
    };

    rsx! {
        div {
            class: "note-box note-out",
            // ใช้ onclick เพื่อให้คลิกการ์ดแล้วเปลี่ยนหน้า
            onclick: move |_| {
                navigator.push(Route::ErrorPage {});
            },
            div {
                img {
                    src: "{props.image}",
                    alt: "Image",
                    max_width: "280px",
                    max_height: "256px",
                    class: "note-image"
                }
            }
            div { class: "note-desc",

                div {
                    h2 { class: "note-text", "{props.title}" }
                    p { class: "line-clamping", "{props.summary}" }
                }

                div { class: "note-icon",
                    div { id: "note-author-bar",

                        div { id: "note-author",
                            img { class: "note-profile-image",
                                src: "{author_image_url}",
                                alt: "Profile Icon"
                            }
                            div { class: "author-info",
                                h3 { "{props.author_name}" }
                                p { "{formatted_date}" }
                            }
                        }

                        img { class: "mark-icon",
                            src: "{_MARK}",
                            alt: "Bookmark Icon"
                        }
                    }
                }

            }
        }
    }
}
