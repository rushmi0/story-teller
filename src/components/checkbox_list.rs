#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};

use crate::components::search_bar::SearchBar;
use crate::styles::checkbox_list_style::STYLE;

const _ICON_FILTER: &str = manganis::mg!(file("src/assets/filter-icon.svg"));

#[component]
pub fn Filter() -> Element {
    rsx! {
        style { {STYLE} }
        SearchBar {}
        div { class: "checkbox-container col-xs-12 col-sm-3 col-lg-1",
            div { class: "checkbox-sidebar col-xs-11",

                div { class: "checkbox-pt",
                    img { src: "{_ICON_FILTER}" }
                    h3 { class: "header", "General" }
                    ul { class: "detail",
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "chill",
                                onclick: |_| info!("chill checkbox clicked")
                            }
                            label { class: "filter-label", "Chill" }
                        }
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "dramatic",
                                onclick: |_| info!("dramatic checkbox clicked")
                            }
                            label { class: "filter-label", "Dramatic" }
                        }
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "happy",
                                onclick: |_| info!("happy checkbox clicked")
                            }
                            label { class: "filter-label", "Happy" }
                        }
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "sad",
                                onclick: |_| info!("sad checkbox clicked")
                            }
                            label { class: "filter-label", "Sad" }
                        }
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "hopeful",
                                onclick: |_| info!("hopeful checkbox clicked")
                            }
                            label { class: "filter-label", "Hopeful" }
                        }
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "fantasy",
                                onclick: |_| info!("fantasy checkbox clicked")
                            }
                            label { class: "filter-label", "Fantasy" }
                        }
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "romantic",
                                onclick: |_| info!("romantic checkbox clicked")
                            }
                            label { class: "filter-label", "Romantic" }
                        }
                        li {
                            input { class: "filter-checkbox", r#type: "checkbox", id: "relaxing",
                                onclick: |_| info!("relaxing checkbox clicked")
                            }
                            label { class: "filter-label", "Relaxing" }
                        }
                    }
                }

            }

        }
    }
}
