#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::styles::article_style::STYLE;

const _IMG: manganis::ImageAsset = manganis::mg!(image("./src/assets/img_5.jpg"));
const _ICON: &str = manganis::mg!(file("src/assets/logo.svg"));
const _CALENDAR: &str = manganis::mg!(file("src/assets/date.svg"));
const _CATEGORY: &str = manganis::mg!(file("src/assets/category.svg"));
const _COMMENT: &str = manganis::mg!(file("src/assets/comment.svg"));

#[component]
pub fn Article() -> Element {
    rsx! {
        style { {STYLE} }
        div { class: "article-box",

            div { class: "article-field",

                div { class: "article-field-text-title",
                    "How to Spend the Perfect Day on Croatia’s Most Magical Island"
                }

                div { class: "article-field-image-header",
                    img { class: "field-image-box", src: "{_IMG}", max_height: "420px", }

                    div { class: "field-pt",
                        div { class: "article-field-icons",
                            div { class: "field-icon-box", img { src: "{_CALENDAR}" }, "July 14 , 2022" }
                            div { class: "field-icon-box", img { src: "{_COMMENT}" }, "comments : 35" }
                            div { class: "field-icon-box", img { src: "{_CATEGORY}" }, "Category : Sport" }
                        }
                    }

                }

                div { class: "article-field-text",
                    "Le Lorem Ipsum est simplement du faux texte employé dans la composition et la mise en page avant impression. Le Lorem Ipsum est le faux texte standard de l'imprimerie depuis les années 1500, quand un imprimeur anonyme assembla ensemble des morceaux de texte pour réaliser un livre spécimen de polices de texte. Il n'a pas fait que survivre cinq siècles, mais s'est aussi adapté à la bureautique informatique, sans que son contenu n'en Ang Lorem Ipsum ay ginagamit na modelo ng industriya ng pagpriprint at pagtytypeset. Ang Lorem Ipsum ang naging regular na modelo simula pa noong 1500s, noong may isang di kilalang manlilimbag and kumuha ng galley ng type at ginulo ang pagkaka-ayos nito upang makagawa ng libro ng mga type specimen. Nalagpasan nito hindi lang limang siglo, kundi nalagpasan din nito ang paglaganap ng electronic typesetting at nanatiling parehas. Sumikat ito noong 1960s kasabay ng pag labas ng Letraset sheets na mayroong mga talata ng Lorem Ipsum, at kamakailan lang sa mga desktop publishing software tulad ng Aldus Pagemaker ginamit ang mga bersyon ng Lorem Ipsum."
                }

            }

        }


    }
}

