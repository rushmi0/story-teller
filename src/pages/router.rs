use dioxus::prelude::*;
use crate::pages::{HomePage, ArticlePage, ErrorPage};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/story/id/:event_id")]
    ArticlePage { event_id: String },
    #[route("/error")]
    ErrorPage {},
}
