#![allow(non_snake_case)]

use std::time::Duration;
use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use nostr_sdk::{
    EventSource,
    PublicKey,
    Filter,
    Event,
    Kind,
    FromBech32
};
use crate::components::anim::EllipsisLoading;
use crate::components::StoryCard;
use crate::nostr::nostr_client::NostrClient;
use crate::styles::story_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_5.jpg"));


#[derive(Debug, Clone)]
struct StoryData {
    image: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    published_at: Option<String>
}

// ฟังก์ชันเพื่อดึงข้อมูลจาก event tags
fn extract_tags(event: Event) -> StoryData {
    let tags_to_find = ["image", "title", "summary", "published_at"];
    let mut image: Option<String> = None;
    let mut title: Option<String> = None;
    let mut summary: Option<String> = None;
    let mut published_at: Option<String> = None;

    // วนซ้ำผ่าน tags ของ event
    for tag in event.tags.iter() {
        let tag_data = tag.as_vec();
        // ตรวจสอบว่า tag มีข้อมูลมากกว่า 1 และมี prefix ที่กำหนด
        if tag_data.len() > 1 && tags_to_find.contains(&&**&tag_data[0]) {
            match tag_data[0].as_str() {
                "image" => image = Some(tag_data[1].to_string()),
                "title" => title = Some(tag_data[1].to_string()),
                "summary" => summary = Some(tag_data[1].to_string()),
                "published_at" => published_at = Some(tag_data[1].to_string()),
                _ => {}
            }
        }
    }

    StoryData { image, title, summary, published_at }
}

#[component]
pub fn Story() -> Element {

    // สร้างตัวแปร signal เพื่อเก็บรายการ events
    let events_signal: Signal<Vec<Event>> = use_signal(Vec::new);

    // สร้างตัวแปร signal ใหม่เพื่อเก็บรายการ StoryData
    let story_data_signal: Signal<Vec<StoryData>> = use_signal(Vec::new);

    use_future({
        // clone signals เพื่อใช้ใน async block
        let mut events_signal = events_signal.clone();
        let mut story_data_signal = story_data_signal.clone();

        move || async move {
            let client = NostrClient::setup_and_connect().await.expect("Failed to setup client");

            let author = FromBech32::from_bech32("npub1mqcwu7muxz3kfvfyfdme47a579t8x0lm3jrjx5yxuf4sknnpe43q7rnz85").expect("Invalid author key");

            let _public_key = PublicKey::from_bech32(
                "npub1drvpzev3syqt0kjrls50050uzf25gehpz9vgdw08hvex7e0vgfeq0eseet",
            ).unwrap();
            //info!("Author: {}", author);

            let filter = Filter::new()
                .kind(Kind::LongFormTextNote)
                .authors(vec![author]);

            let events = client
                .get_events_of(
                    vec![filter],
                    EventSource::relays(Some(Duration::from_secs(10))),
                )
                .await;

            if let Ok(events) = events {
                //info!("Events received: {:?}", events);
                // อัพเดตค่า signal ด้วยรายการ events ที่ได้รับ
                events_signal.set(events.clone());

                // สร้าง StoryData จาก events และอัพเดต story_data_signal
                let stories: Vec<StoryData> = events.into_iter().map(extract_tags).collect();
                story_data_signal.set(stories);

                // ตัดการเชื่อมต่อหลังจากได้รับข้อมูล
                client.disconnect().await.expect("Failed to disconnect");
            } else {
                info!("Failed to retrieve events");
            }
        }
    });

    for story in story_data_signal.iter() {
        info!("Story data: {:?}", story);
    }

    rsx! {
        style { {STYLE} }
        div { class: "note-container",
            // StoryCard { name: "Eimi Fukada", image: _IMG, title: "this title note event", summary: "Matagal na nating alam na nakukuha ang atensyon ng nagbabasa ang nababasa na nilalaman ng", published_at: "1296962229" }
            // StoryCard { name: "Eimi Fukada", image: _IMG, title: "this title note event", summary: "Matagal na nating alam na nakukuha ang atensyon ng nagbabasa ang nababasa na nilalaman ng", published_at: "1296962229" }
            // StoryCard { name: "Eimi Fukada", image: _IMG, title: "this title note event", summary: "Matagal na nating alam na nakukuha ang atensyon ng nagbabasa ang nababasa na nilalaman ng", published_at: "1296962229" }
            // StoryCard { name: "Eimi Fukada", image: _IMG, title: "this title note event", summary: "Matagal na nating alam na nakukuha ang atensyon ng nagbabasa ang nababasa na nilalaman ng", published_at: "1296962229" }
            // StoryCard { name: "Eimi Fukada", image: _IMG, title: "this title note event", summary: "Matagal na nating alam na nakukuha ang atensyon ng nagbabasa ang nababasa na nilalaman ng", published_at: "1296962229" }
            // StoryCard { name: "Eimi Fukada", image: _IMG, title: "this title note event", summary: "Matagal na nating alam na nakukuha ang atensyon ng nagbabasa ang nababasa na nilalaman ng", published_at: "1296962229" }
            // StoryCard { name: "Eimi Fukada", image: _IMG, title: "this title note event", summary: "Matagal na nating alam na nakukuha ang atensyon ng nagbabasa ang nababasa na nilalaman ng", published_at: "1296962229" }


            if story_data_signal.read().is_empty() {
                EllipsisLoading {}
            } else {
                for story in story_data_signal.iter() {
                    StoryCard {
                        name: "Eimi Fukada",
                        image: story.image.clone().unwrap_or_else(|| _IMG.to_string()),
                        title: story.title.clone().unwrap_or_default(),
                        summary: story.summary.clone().unwrap_or_default(),
                        published_at: story.published_at.clone().unwrap_or_default()
                    }
                }
            }

        }
    }
}
