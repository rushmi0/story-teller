#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::components::{Banner, SearchBar};
use crate::components::content::{Article, ArticleAuthor};
use crate::styles::layout_style::STYLE;

#[derive(PartialEq, Props, Clone)]
pub struct ArticleProps {
    note_id: String,
    // author_name: String,
    // author_image: String,
}


#[component]
pub fn ArticlePage(props: ArticleProps) -> Element {

    let note_id = &props.note_id;
    // let author_name = &props.author_name;
    // let author_image = &props.author_image;

    rsx! {
        style { {STYLE} }
        Banner {}
        SearchBar {}
        h1 { "{note_id}" }
        div { class: "container" ,
            div { class: "control-box",
                //style: "background-color:yellow;",

                div { class: "col-lg-8 col-sm-8",
                    //style: "background-color:red;", // สำหรับ Debug
                    Article {}
                }
                div { class: "col-lg-4 col-sm-4",
                    //style: "background-color:green;", // สำหรับ Debug
                    ArticleAuthor {}
                }
            }
        }


    }
}
