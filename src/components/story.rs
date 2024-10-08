#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::StoryCard;
use crate::styles::story_style::STYLE;

#[component]
pub fn Story() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "note-container",
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
            StoryCard {}
        }
    }
}