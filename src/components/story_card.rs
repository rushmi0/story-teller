#![allow(non_snake_case)]

use dioxus::prelude::*;
use chrono::{NaiveDateTime, TimeZone, Utc};
use crate::pages::router::Route;

const _PLAY: &str = manganis::mg!(file("src/assets/play.svg"));
const _FAV: &str = manganis::mg!(file("src/assets/fav.svg"));
const _MARK: &str = manganis::mg!(file("src/assets/mark.svg"));

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_4.jpg"));

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
    // ฟังก์ชันแปลง Unix timestamp เป็นวันที่ในรูปแบบ "July 14, 2022"
    fn format_unix_to_date(unix_timestamp: &str) -> String {
        let timestamp = unix_timestamp.parse::<i64>().unwrap_or(0);
        let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
        let datetime = Utc.from_utc_datetime(&naive); // แปลงเป็น datetime ที่ใช้ UTC

        // แปลงเป็นรูปแบบ "July 14, 2022"
        datetime.format("%B %d, %Y").to_string()
    }

    // แปลง published_at จาก Unix timestamp เป็นวันที่
    let formatted_date = format_unix_to_date(&props.published_at);

    // ตรวจสอบ author_image ว่า URL ใช้งานได้หรือไม่
    let mut author_image_future = use_resource(|| async move {
        match reqwest::get(&props.author_image).await {
            Ok(response) if response.status().is_success() => Ok(props.author_image.clone()),
            _ => Err(()),
        }
    });

    // ดึงค่าจาก future
    let author_image_src = match author_image_future.read_unchecked() {
        Some(Ok(url)) => url,
        _ => String::from(_IMG.to_string()),
    };

    rsx! {
        div {
            class: "note-box note-out",
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
                                src: "{author_image_src}",
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
