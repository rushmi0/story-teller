#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::search_bar_style::STYLE;

#[component]
pub fn SearchBar() -> Element {
    rsx! {
        style { {STYLE} }
        div { id: "search-pt",
            div { class: "search-bar col-lg-7 col-xs-9",
                div { class: "input-container",

                    input { class: "card",
                        r#type: "text",
                        placeholder: "Search"
                    }

                    // Search Icon ในช่องใส่ข้อมูล
                    svg { class: "searchIcon",
                        view_box: "0 0 512 512",
                        path {
                            d: "M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z",
                        }
                    }

                }
            }
        }
    }
}
