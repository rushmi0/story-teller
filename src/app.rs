#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::router::Route;
use crate::styles::screen_config::STYLE;

// use nostr_sdk::nips::nip07;
// use nostr_sdk::ToBech32;
// use web_sys::console;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
const _CONFIG: &str = manganis::mg!(file("src/main.css"));
const _ICON: &str = manganis::mg!(file("src/assets/icon.svg"));

#[component]
pub fn App() -> Element {
    // use_future(move || async move {
    //     let signer = nip07::Nip07Signer::new().expect("extension exits");
    //     let npub = signer.get_public_key().await.unwrap();
    //     console::log_1(&npub.to_bech32().unwrap().to_string().into());
    // });
    rsx! {
        link { rel: "icon", href: _ICON }
        link { rel: "stylesheet", href: _CONFIG }
        style { {STYLE} }
        Router::<Route> { }
    }
}
