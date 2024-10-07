#![allow(non_snake_case)]

use dioxus::prelude::*;

const _IMG: &str = manganis::mg!(file("src/assets/img_4.jpg"));
const _PLAY: &str = manganis::mg!(file("src/assets/play.svg"));
const _FAV: &str = manganis::mg!(file("src/assets/fav.svg"));

//const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_5.jpg"));
// const _PLAY: manganis::ImageAsset = manganis::mg!(image("./src/assets/play.svg"));
// const _FAV: manganis::ImageAsset = manganis::mg!(image("s./rc/assets/fav.svg"));

#[component]
pub fn StoryCard() -> Element {
    rsx! {
        div { class: "card",
            div {
                img {
                    src: "{_IMG}",
                    alt: "Image",
                    max_width: "252px",
                    max_height: "256px",
                    class: "card-image"
                }
            }
            div { class: "card-desc",
                h2 { "title card" }
                p { class: "card-text", "Lorem Ipsum ay ginagamit na modelo" }
                hr {}
                div { class: "card-icon",
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
