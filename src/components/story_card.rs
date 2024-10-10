// story_card.rs
#![allow(non_snake_case)]

use dioxus::prelude::*;

const _PLAY: &str = manganis::mg!(file("src/assets/play.svg"));
const _FAV: &str = manganis::mg!(file("src/assets/fav.svg"));


#[derive(PartialEq, Props, Clone)]
pub struct StoryCardProps {
    image: String,
}

#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {
    rsx! {
        div { class: "note-box note-out",
            div {
                img {
                    src: "{props.image}",
                    alt: "Image",
                    max_width: "252px",
                    max_height: "256px",
                    class: "note-image"
                }
            }
            div { class: "note-desc",
                h2 { "title note" }
                p { class: "note-text", "Lorem Ipsum ay ginagamit na modelo" }
                hr {}
                div { class: "note-icon",
                    img {
                        src: "{_PLAY}",
                        alt: "Play Icon"
                    }
                    img {
                        src: "{_FAV}",
                        alt: "Favorite Icon"
                    }
                }
            }
        }
    }
}
