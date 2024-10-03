#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::filter_style::STYLE;

const _ICON_FILTER: &str = manganis::mg!(file("src/assets/filter-icon.svg"));

#[component]
pub fn Filter() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "filter-container col-xs-12 col-sm-12 col-lg-10", id: "filterAndSearch",
            div { class: "filter-sidebar",

                div {
                    img {
                        src: "{_ICON_FILTER}"
                    }
                }

                h3 { class: "custom-font-h1", "General" }
                ul { class: "custom-font-h2",
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "chill" }
                        label { class: "filter-label", "Chill" }
                    }
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "dramatic" }
                        label { class: "filter-label", "Dramatic" }
                    }
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "happy" }
                        label { class: "filter-label", "Happy" }
                    }
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "sad" }
                        label { class: "filter-label", "Sad" }
                    }
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "hopeful" }
                        label { class: "filter-label", "Hopeful" }
                    }
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "fantasy" }
                        label { class: "filter-label", "Fantasy" }
                    }
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "romantic" }
                        label { class: "filter-label", "Romantic" }
                    }
                    li {
                        input { class: "filter-checkbox", r#type: "checkbox", id: "relaxing" }
                        label { class: "filter-label", "Relaxing" }
                    }
                }
            }
            div { class: "search-bar",
                input { r#type: "text", placeholder: "Search Music or Backsound" }
            }
        }
    }
}
