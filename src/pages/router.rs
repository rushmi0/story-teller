use dioxus::prelude::*;
use crate::pages::{HomePage, ArticlePage, ErrorPage};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/article")]
    ArticlePage,
    #[route("/error")]
    ErrorPage {},
}
