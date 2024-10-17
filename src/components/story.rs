#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use nostr_sdk::{
    serde_json, Client, Event, EventSource, Filter, FromBech32, Kind, Metadata,
    PublicKey,
};
use std::time::Duration;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlImageElement;

use crate::components::anim::EllipsisLoading;
use crate::components::StoryCard;
use crate::nostr::NostrClient;
use crate::styles::story_style::STYLE;

const _IMG: manganis::ImageAsset =
    manganis::mg!(image("./src/assets/Untitled.webp"));

/// โครงสร้างข้อมูลสำหรับเก็บข้อมูล
/// Struct นี้จะเก็บข้อมูลเช่น id ของ story, รูปภาพ, ชื่อเรื่อง, บทสรุป, เวลาที่เผยแพร่,
/// ชื่อและรูปภาพของผู้เขียน
#[derive(Debug, Clone)]
struct StoryData {
    note_id: Option<String>,      // ค่า Event id ของ note
    image: Option<String>,        // รูปภาพของ note
    title: Option<String>,        // ชื่อของ note
    summary: Option<String>,      // สรุปของ note
    published_at: Option<String>, // เวลาที่เผยแพร่
    author_name: Option<String>,  // ชื่อผู้เขียน
    author_image: Option<String>, // รูปภาพผู้เขียน
}

pub async fn check_image(url: &str) -> bool {
    // Create a new HtmlImageElement
    let img = HtmlImageElement::new().unwrap();
    img.set_src(url);

    // Create a promise to check if the image loads successfully
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let img_clone = img.clone();
        let resolve = resolve.clone();
        let reject = reject.clone();

        let onload = Closure::wrap(Box::new(move || {
            resolve.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        let onerror = Closure::wrap(Box::new(move || {
            reject.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        img_clone.set_onload(Some(onload.as_ref().unchecked_ref()));
        img_clone.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        onload.forget();
        onerror.forget();
    });

    // Await the promise
    match JsFuture::from(promise).await {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// ฟังก์ชัน `extract_tags`
/// ฟังก์ชันนี้ใช้สำหรับดึงข้อมูลจาก tags ของ event ที่ได้รับ เช่น รูปภาพ ชื่อเรื่อง สรุป และเวลาที่เผยแพร่
/// และนำข้อมูลของผู้เขียนจาก parameter `author_name` และ `author_image` เข้ามาประกอบ
///
/// # Arguments
/// - `event`: ข้อมูล event ที่ได้รับจากเครือข่าย Nostr
/// - `author_name`: ชื่อผู้เขียน (เป็น Option เผื่อว่าอาจจะไม่มีข้อมูล)
/// - `author_image`: รูปภาพของผู้เขียน (เป็น Option เช่นกัน)
///
/// # Returns
/// คืนค่าเป็น Struct `StoryData` ที่ประกอบไปด้วยข้อมูล story ทั้งหมด
async fn extract_tags(
    event: Event,
    author_name: Option<String>,
    author_image: Option<String>,
) -> StoryData {
    // กำหนดประเภทของ tags ที่ต้องการหา
    let tags_to_find = ["image", "title", "summary", "published_at"];

    let mut image: Option<String> = None; // ตัวแปรเก็บค่า image
    let mut title: Option<String> = None; // ตัวแปรเก็บค่า title
    let mut summary: Option<String> = None; // ตัวแปรเก็บค่า summary
    let mut published_at: Option<String> = None; // ตัวแปรเก็บค่า published_at
    let note_id = Some(event.id.to_hex()); // แปลง id ของ event เป็นรูปแบบ hex และเก็บใน note_id

    // วนซ้ำเพื่อตรวจสอบ tags ภายใน event
    for tag in event.tags.iter() {
        let tag_data = tag.as_slice(); // แปลง tag เป็นรูปแบบเวกเตอร์
                                       // ตรวจสอบว่ามีข้อมูลใน tag และเป็น tag ที่เราต้องการหรือไม่
        if tag_data.len() > 1 && tags_to_find.contains(&tag_data[0].as_str()) {
            match tag_data[0].as_str() {
                "image" => {
                    image = {
                        let image = tag_data[1].to_string();
                        if check_image(&image).await {
                            Some(image)
                        } else {
                            Some(_IMG.to_string())
                        }
                    }
                } // หากเป็น image, เก็บข้อมูล
                "title" => title = Some(tag_data[1].to_string()), // หากเป็น title, เก็บข้อมูล
                "summary" => summary = Some(tag_data[1].to_string()), // หากเป็น summary, เก็บข้อมูล
                "published_at" => published_at = Some(tag_data[1].to_string()), // หากเป็น published_at, เก็บข้อมูล
                _ => {}
            }
        }
    }

    // คืนค่า StoryData ที่ประกอบไปด้วยข้อมูลที่เราดึงมาได้จาก tags และข้อมูลผู้เขียน
    StoryData {
        note_id,
        image,
        title,
        summary,
        published_at,
        author_name,
        author_image,
    }
}

/// ฟังก์ชัน `Story`
/// เป็น component ที่ทำหน้าที่ดึงข้อมูล NIP-23 (Long-form Content) จากเครือข่าย Nostr
/// และแสดงผลออกมาเป็นรายการของ story โดยใช้ `StoryCard` component
#[component]
pub fn Story() -> Element {
    // สร้าง signal เพื่อเก็บ events ที่ได้จากการดึงข้อมูล
    let events_signal: Signal<Vec<Event>> = use_signal(Vec::new);

    // เก็บข้อมูล StoryData ที่ประมวลผลแล้วจาก `event`
    let story_data_signal: Signal<Vec<StoryData>> = use_signal(Vec::new);

    use_future({
        // clone ตัวแปร signal ที่สร้างขึ้นเพื่อใช้ใน future
        let mut events_signal = events_signal.clone();

        // clone ตัวแปร signal เพื่อเก็บ story data
        let mut story_data_signal = story_data_signal.clone();

        // ฟังก์ชัน async ที่ดึงข้อมูล events จาก Nostr
        move || async move {
            let client = NostrClient::setup_and_connect()
                .await
                .expect("Failed to setup client");

            // Public key ของผู้ใช้งาน (ระบุเป็นค่าเบื้องต้น)
            //let _public_key = PublicKey::from_bech32("npub1drvpzev3syqt0kjrls50050uzf25gehpz9vgdw08hvex7e0vgfeq0eseet").unwrap();

            // สร้าง filter สำหรับดึงข้อมูล event ที่เป็นประเภท LongFormTextNote
            let filter = Filter::new().kind(Kind::LongFormTextNote);

            // ดึงข้อมูล events จากเครือข่ายด้วย filter ที่เรากำหนด
            let events = client
                .get_events_of(
                    vec![filter],
                    EventSource::relays(Some(Duration::from_secs(10))),
                )
                .await;

            // ถ้าการดึงข้อมูลสำเร็จ
            if let Ok(events) = events {
                // อัพเดตค่า signal ด้วยรายการ events ที่ได้รับมา
                events_signal.set(events.clone());

                // สร้างเวกเตอร์เปล่าเพื่อเก็บ StoryData
                let mut stories: Vec<StoryData> = Vec::new();

                // วนซ้ำผ่านแต่ละ event เพื่อดึงข้อมูล Metadata ของผู้เขียน
                for event in events.iter() {
                    // สร้าง filter เพื่อดึง metadata ของผู้เขียน
                    let metadata_filter =
                        Filter::new().author(event.pubkey).kind(Kind::Metadata);

                    let metadata_events = client
                        .get_events_of(
                            vec![metadata_filter],
                            EventSource::relays(Some(Duration::from_secs(10))),
                        )
                        .await;

                    let mut author_name = None; // ตัวแปรเก็บชื่อผู้เขียน
                    let mut author_image = None; // ตัวแปรเก็บรูปภาพของผู้เขียน

                    // ถ้าการดึงข้อมูล Metadata สำเร็จ
                    if let Ok(metadata_events) = metadata_events {
                        let mut ts = 0u64;
                        for metadata_event in metadata_events {
                            let user_metadata: Metadata =
                                serde_json::from_str::<Metadata>(
                                    &*metadata_event.content,
                                )
                                .unwrap();

                            if !metadata_event.is_expired()
                                && ts.lt(&metadata_event.created_at.as_u64())
                            {
                                // ดึงข้อมูลชื่อและรูปภาพของผู้เขียนจาก Metadata
                                author_name = user_metadata.name.clone();
                                author_image = user_metadata.picture.clone();
                            }
                            ts = metadata_event.created_at.as_u64();

                            //info!("User Metadata - Name: {:?}, Picture: {:?}", author_name, author_image);
                        }
                    }

                    if author_image.is_some() {
                        let result =
                            check_image(author_image.clone().unwrap().as_str())
                                .await;
                        if !result {
                            let pk = event.pubkey.to_hex();
                            let image_proxy = format!(
                            "https://media.nostr.band/thumbs/{}/{}-picture-64",
                            &pk[60..],
                            pk);
                            let result = check_image(&image_proxy).await;
                            if result {
                                author_image = Some(image_proxy);
                            } else {
                                author_image = Some(_IMG.to_string());
                            }
                        }
                    } else {
                        author_image = Some(_IMG.to_string());
                    }

                    // ใช้ฟังก์ชัน extract_tags เพื่อดึงข้อมูลจาก event และ Metadata ของผู้เขียน
                    let story = extract_tags(
                        event.clone(),
                        author_name.clone(),
                        author_image.clone(),
                    )
                    .await;
                    stories.push(story);
                    story_data_signal.set(stories.clone());
                }

                // อัพเดตค่า signal ด้วยข้อมูล story ที่ประมวลผลแล้ว

                // ตัดการเชื่อมต่อหลังจากดึงข้อมูลเสร็จ
                client.disconnect().await.expect("Failed to disconnect");
            } else {
                info!("Failed to retrieve events");
            }
        }
    });

    rsx! {
        style { {STYLE} }
        div { class: "note-container",

            // ถ้าข้อมูลใน story_data_signal ยังว่างอยู่ ให้แสดง EllipsisLoading
            if story_data_signal.read().is_empty() {
                EllipsisLoading {}
            } else {
                // ถ้ามีข้อมูลแล้ว ให้วนซ้ำแสดงผลแต่ละ story โดยใช้ StoryCard component
                for story in story_data_signal.iter() {
                    StoryCard {
                        note_id: story.note_id.clone().unwrap_or_default(),
                        name: story.author_name.clone().unwrap_or("Unknown Author".to_string()),
                        image: story.image.clone().unwrap_or_else(|| _IMG.to_string()),
                        title: story.title.clone().unwrap_or_default(),
                        summary: story.summary.clone().unwrap_or_default(),
                        published_at: story.published_at.clone().unwrap_or_default(),
                        author_name: story.author_name.clone().unwrap_or_default(),
                        author_image: story.author_image.clone().unwrap_or_default(),
                    }
                }
            }
        }
    }
}
