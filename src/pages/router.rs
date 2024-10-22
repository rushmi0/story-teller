use dioxus::prelude::*;
use crate::pages::{HomePage, ArticlePage, ErrorPage};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/article/id/:note_id")]
    ArticlePage { note_id: String },
    #[route("/error")]
    ErrorPage {},
}
