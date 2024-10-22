#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use pulldown_cmark::{Parser, Options, html};
use crate::components::story_card::format_unix_to_date;
use crate::styles::article_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_5.jpg"));
const _ICON: &str = manganis::mg!(file("src/assets/logo.svg"));
const _CALENDAR: &str = manganis::mg!(file("src/assets/date.svg"));
const _CATEGORY: &str = manganis::mg!(file("src/assets/category.svg"));
const _COMMENT: &str = manganis::mg!(file("src/assets/comment.svg"));

#[derive(PartialEq, Props, Clone)]
pub struct ArticleProps {
    image: String,
    title: String,
    summary: String,
    article: String,
    published_at: String,
}

fn markdown_to_html(markdown_input: &str) -> String {
    // เพิ่ม options สำหรับตารางและ fenced code blocks
    let options = Options::ENABLE_TABLES;
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[component]
pub fn Article(props: ArticleProps) -> Element {
    let formatted_date = format_unix_to_date(&props.published_at);
    let article = markdown_to_html(&props.article);  // แปลง markdown เป็น html
    info!("{}", article);

    rsx! {
        style { {STYLE} }
        div { class: "article-box",

            div { class: "article-field",
                div { class: "article-field-text-title", "{props.title}" }
                div { class: "article-field-image-header",
                    img { class: "field-image-box", src: "{props.image}", max_height: "420px", }
                    div { class: "field-pt",
                        div { class: "article-field-icons",
                            div { class: "field-icon-box", img { src: "{_CALENDAR}" }, "{formatted_date}" }
                            div { class: "field-icon-box", img { src: "{_COMMENT}" }, "comments : 35" }
                            div { class: "field-icon-box", img { src: "{_CATEGORY}" }, "Category : Sport" }
                        }
                    }
                }

                div { class: "article-field-text",
                    dangerous_inner_html: "{article}"  // ใช้ html ที่แปลงจาก markdown
                }

            }

        }
    }
}
