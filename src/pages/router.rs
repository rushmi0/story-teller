use dioxus::prelude::*;
use crate::pages::{HomePage,ErrorPage};

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/error")]
    ErrorPage {},
}
