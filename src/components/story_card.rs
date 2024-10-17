#![allow(non_snake_case)]


use dioxus::prelude::*;
use chrono::{NaiveDateTime, TimeZone, Utc};
use crate::pages::router::Route;

// กำหนดค่าคงที่สำหรับเส้นทางของไฟล์ SVG และรูปภาพที่จะใช้ใน component นี้
const _MARK: &str = manganis::mg!(file("src/assets/mark.svg"));
const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/Untitled.webp"));

/// โครงสร้าง StoryCardProps ใช้สำหรับรับข้อมูลที่จำเป็นในการแสดงการ์ด
/// - note_id: ค่า Event id ของ note
/// - name: ชื่อเรื่อง
/// - image: รูปภาพของ note
/// - title: หัวข้อของ note
/// - summary: บทสรุปของ note
/// - published_at: เวลาที่เผยแพร่ (ในรูปแบบ Unix timestamp)
/// - author_name: ชื่อผู้เขียน
/// - author_image: รูปภาพของผู้เขียน
#[derive(PartialEq, Props, Clone)]
pub struct StoryCardProps {
    note_id: String,
    name: String,
    image: String,
    title: String,
    summary: String,
    published_at: String,
    author_name: String,
    author_image: String,
}

/// ฟังก์ชัน `StoryCard`
/// ใช้สำหรับแสดงการ์ดของเรื่องราวที่มีรูปภาพ ชื่อเรื่อง ผู้เขียน และวันเวลาที่เผยแพร่
/// เมื่อคลิกที่การ์ดจะนำผู้ใช้ไปยังหน้าข้อผิดพลาด (Error Page)
#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {

    // สร้าง navigator เพื่อให้สามารถเปลี่ยนหน้าเมื่อคลิกการ์ด
    let navigator: Navigator = use_navigator();

    /// ใช้แปลง Unix timestamp ให้เป็นรูปแบบวันที่ เช่น "July 14, 2022"
    /// - unix_timestamp: เวลาในรูปแบบ Unix timestamp ที่จะถูกแปลง
    /// คืนค่าเป็นสตริงวันที่ที่อ่านง่าย
    fn format_unix_to_date(unix_timestamp: &str) -> String {
        let timestamp = unix_timestamp.parse::<i64>().unwrap_or(0);
        let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
        let datetime = Utc.from_utc_datetime(&naive);

        // แปลงวันที่เป็นรูปแบบ "July 14, 2022"
        datetime.format("%B %d, %Y").to_string()
    }

    // แปลง `published_at` ของ note ให้เป็นวันที่ในรูปแบบที่อ่านง่าย
    let formatted_date = format_unix_to_date(&props.published_at);

    rsx! {
        div { class: "note-box note-out",

            // ตั้ง onclick event สำหรับคลิกการ์ดแล้วนำไปยังหน้า ErrorPage
            onclick: move |_| {
                navigator.push(Route::ErrorPage {});
            },

            // ส่วนของรูปภาพของ note
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

                div { class: "textbox-note",
                    h2 { class: "note-text", "{props.title}" }
                    p { class: "line-clamping", "{props.summary}" }
                }

                div { class: "note-icon",
                    div { id: "note-author-bar",

                        // ส่วนของข้อมูลผู้เขียน note
                        div { id: "note-author",
                            img { class: "note-profile-image",
                                src: "{props.author_image}",
                                alt: "Profile image"
                            }

                            div { class: "author-info",
                                h3 { "{props.author_name}" }
                                p { "{formatted_date}" }
                            }
                        }

                        // ไอคอนบุ๊คมาร์ค (Bookmark Icon)
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
