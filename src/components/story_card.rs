#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::router::Route;

const _PLAY: &str = manganis::mg!(file("src/assets/play.svg"));
const _FAV: &str = manganis::mg!(file("src/assets/fav.svg"));


#[derive(PartialEq, Props, Clone)]
pub struct StoryCardProps {
    image: String,
    title: String,
    summary: String,
}

#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {
    let navigator: Navigator = use_navigator();

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
                    max_width: "252px",
                    max_height: "256px",
                    class: "note-image"
                }
            }
            div { class: "note-desc",
                h2 { "{props.title}" }
                p { class: "note-text", "{props.summary}" }
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
