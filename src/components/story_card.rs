#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::router::Route;

const _PLAY: &str = manganis::mg!(file("src/assets/play.svg"));
const _FAV: &str = manganis::mg!(file("src/assets/fav.svg"));
const _MARK: &str = manganis::mg!(file("src/assets/mark.svg"));

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_4.jpg"));

#[derive(PartialEq, Props, Clone)]
pub struct StoryCardProps {
    name: String,
    image: String,
    title: String,
    summary: String,
    published_at: String
}

#[component]
pub fn StoryCard(props: StoryCardProps) -> Element {
    let navigator: Navigator = use_navigator();

    rsx! {
        div {
            class: "note-box note-out",
            // ใช้ onclick เพื่อให้คลิกการ์ดแล้วเปลี่ยนหน้า
            // onclick: move |_| {
            //     navigator.push(Route::ErrorPage {});
            // },
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
                                src: "{_IMG}",
                                alt: "Profile Icon"
                            }
                            div { class: "author-info",
                                h3 { "{props.name}" }
                                p { "{props.published_at}" }
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