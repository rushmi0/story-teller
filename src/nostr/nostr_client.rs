// use nostr_sdk::{Client, EventSource, Filter};
// use std::time::Duration;
// use std::fs;
// use dioxus_logger::tracing::info;
// use nostr_sdk::Event;
// use serde::{Deserialize};
//
// #[derive(Deserialize)]
// struct RelayList {
//     relays: Vec<String>,
// }
//
// #[derive(Clone)]
// pub struct NostrClient {
//     client: Client,
// }
//
// impl NostrClient {
//
//     pub fn new() -> Self {
//         let client = Client::default();
//         NostrClient { client }
//     }
//
//     pub async fn add_relays(&mut self, urls: Vec<&str>) {
//         for url in urls {
//             if let Err(e) = self.client.add_relay(url).await {
//                 info!("Failed to add relay {}: {:?}", url, e);
//             } else {
//                 info!("Successfully added relay: {}", url);
//             }
//         }
//         self.client.connect().await;
//     }
//
//     pub async fn get_events(&self, filter: Filter) -> Result<Vec<Event>, String> {
//         let events = self.client
//             .get_events_of(
//                 vec![filter],
//                 EventSource::relays(Some(Duration::from_secs(10))),
//             )
//             .await;
//
//         match events {
//             Ok(events) => {
//                 let valid_events: Vec<Event> = events.into_iter().filter_map(|event| Option::from(event)).collect();
//
//                 if valid_events.is_empty() {
//                     Err("No valid events found".to_string())
//                 } else {
//                     Ok(valid_events)
//                 }
//             }
//             Err(e) => Err(format!("Error retrieving events: {:?}", e)),
//         }
//     }
//
//
//
// }
