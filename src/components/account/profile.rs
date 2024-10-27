#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::Story;
use crate::styles::profile_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/banner.jpg"));
const _PROFILE: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_4.jpg"));
const _EDIT: &str = manganis::mg!(file("src/assets/edit.svg"));

#[component]
pub fn Profile() -> Element {
    rsx! {
        style { {STYLE} }

        // Profile Card
        div { class: "profile-box",
            div { class: "banner-box col-xs-hidden",
                img { src: "{_IMG}" }
            }

            div { class: "profile-info",
                div { class: "profile-bar",

                    div { class: "profile-field-image",
                        img { src: "{_PROFILE}", alt: "Profile Image" }
                        span { class: "profile-name", "Eimi Fukada" }
                    }

                    div { class: "profile-field-menu",
                        button { class: "menu-btn",
                            r#type: "button",
                            span { "New Post" }
                        }
                        button { class: "menu-btn",
                            r#type: "button",
                            span { "Article List" }
                        }
                    }

                    div { class: "profile-field-edit",
                        button { class: "edit-btn",
                            r#type: "button",
                            img { src: "{_EDIT}", alt: "Edit Icon" }
                            span { "Edit Profile" }
                        }
                    }

                }

            }

        } // End Profile Card

        div { class: "display-contents",
            Story {}
            div { class: "set-display",

            }
        }

    }
}
