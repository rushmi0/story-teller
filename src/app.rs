#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::HomePage;
use crate::styles::screen_config::STYLE;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
const INFRA: &str = manganis::mg!(file("src/main.css"));
const ICON: &str = manganis::mg!(file("src/assets/icon.svg"));


#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "icon", href: ICON }
        link { rel: "stylesheet", href: INFRA }
        style { {STYLE} }
        HomePage {}
    }
}
