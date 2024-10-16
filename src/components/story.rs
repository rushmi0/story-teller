// story.rs

#![allow(non_snake_case)]

use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use dioxus::prelude::*;
use dioxus_logger::tracing::{error, info};
use nostr_sdk::{EventSource, PublicKey, Filter, Event, Kind, FromBech32, Metadata, Client};
use crate::components::anim::EllipsisLoading;
use crate::components::shared::metadata_visibility::UserMetadata;
use crate::components::shared::SharedMetadataVisibility;
use crate::components::StoryCard;
use crate::nostr::nostr_client::NostrClient;
use crate::styles::story_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/Untitled.webp"));

#[derive(Debug, Clone)]
struct StoryData {
    image: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    published_at: Option<String>,
}

#[derive(Debug, Clone)]
struct StoryAuthor {
    note_id: Option<String>,
    author_name: Option<String>,
    author_image: Option<String>,
}

// ฟังก์ชันเพื่อดึงข้อมูลจาก event tags
fn extract_story(event: &Event) -> StoryData {
    let tags_to_find = ["image", "title", "summary", "published_at"];
    let mut image: Option<String> = None;
    let mut title: Option<String> = None;
    let mut summary: Option<String> = None;
    let mut published_at: Option<String> = None;

    for tag in event.tags.iter() {
        let tag_data = tag.as_vec();
        if tag_data.len() > 1 && tags_to_find.contains(&tag_data[0].as_str()) {
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

async fn extract_author(event: &Event, state_metadata: &mut SharedMetadataVisibility, client: &Client) -> Option<StoryAuthor> {
    let pubkey = event.pubkey;
    let filter_metadata = Filter::new().author(pubkey).kind(Kind::Metadata);

    // ใช้ client เพื่อเรียก metadata ของ author
    if let Ok(metadata_events) = client.get_events_of(vec![filter_metadata], EventSource::relays(Some(Duration::from_secs(10)))).await {
        // สมมติว่าเราใช้เฉพาะ event แรกที่ดึงได้เป็น metadata
        if let Some(metadata_event) = metadata_events.get(0) {
            state_metadata.metadata.set(Some(metadata_event.clone()));

            if let Some(metadata) = state_metadata.metadata.read().as_ref() {
                if let Ok(user_metadata) = serde_json::from_str::<UserMetadata>(&metadata.content) {
                    return Some(StoryAuthor {
                        note_id: Some(metadata.id.to_string()),
                        author_name: Some(user_metadata.name),
                        author_image: Some(user_metadata.picture),
                    });
                }
            }
        }
    }

    None
}


#[component]
pub fn Story() -> Element {
    let events_signal: Signal<Vec<Event>> = use_signal(Vec::new);
    let story_data_signal: Signal<Vec<StoryData>> = use_signal(Vec::new);
    let story_author_signal: Signal<Vec<StoryAuthor>> = use_signal(Vec::new);
    let state_metadata = SharedMetadataVisibility::new();

    let state_metadata = Arc::new(Mutex::new(SharedMetadataVisibility::new()));

    use_future({
        let mut events_signal = events_signal.clone();
        let mut story_data_signal = story_data_signal.clone();
        let mut story_author_signal = story_author_signal.clone();
        let state_metadata = state_metadata.clone();

        move || {
            let state_metadata = state_metadata.clone();

            async move {
                let client = NostrClient::setup_and_connect().await.expect("Failed to setup client");
                let author_1 = FromBech32::from_bech32("npub1mqcwu7muxz3kfvfyfdme47a579t8x0lm3jrjx5yxuf4sknnpe43q7rnz85").expect("Invalid author key");
                let author_2 = FromBech32::from_bech32("npub1vm0kq43djwdd4psjgdjgn9z6fm836c35dv7eg7x74z3n3ueq83jqhkxp8e").expect("Invalid author key");

                let filter = Filter::new()
                    .kind(Kind::LongFormTextNote)
                    .authors(vec![author_1, author_2]);

                if let Ok(events) = client.get_events_of(vec![filter], EventSource::relays(Some(Duration::from_secs(10)))).await {
                    events_signal.set(events.clone());

                    let stories: Vec<StoryData> = events.iter().map(extract_story).collect();
                    story_data_signal.set(stories);

                    let authors: Vec<StoryAuthor> = futures::stream::iter(events.iter())
                        .filter_map(|e| {
                            let client = client.clone();
                            let state_metadata = state_metadata.clone();

                            async move {
                                let mut state_metadata_lock = state_metadata.lock().unwrap();
                                extract_author(e, &mut state_metadata_lock, &client).await
                            }
                        })
                        .collect()
                        .await;

                    story_author_signal.set(authors);

                    client.disconnect().await.expect("Failed to disconnect");
                } else {
                    info!("Failed to retrieve events");
                }
            }
        }
    });


    rsx! {
        style { {STYLE} }
        div { class: "note-container",

            if story_data_signal.read().is_empty() {
                EllipsisLoading {}
            } else {

                if !story_data_signal.read().is_empty() && !story_author_signal.read().is_empty() {
                    for (story, author) in story_data_signal.iter().zip(story_author_signal.iter()) {
                        StoryCard {
                            image: story.image.clone().unwrap_or_else(|| _IMG.to_string()),
                            title: story.title.clone().unwrap_or_default(),
                            summary: story.summary.clone().unwrap_or_default(),
                            published_at: story.published_at.clone().unwrap_or_default(),
                            author_name: author.author_name.clone().unwrap_or_else(|| "Unknown Author".to_string()),
                            note_id: author.note_id.clone().unwrap_or_default(),
                            author_image: author.author_image.clone().unwrap_or_default(),
                        }
                    }
                }


            }
        }
    }
}
