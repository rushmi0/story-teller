// story.rs
#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::StoryCard;
use crate::styles::story_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_5.jpg"));

#[component]
pub fn Story() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "note-container",
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
            StoryCard { image: _IMG.to_string() }
        }
    }
}
