mod components;
mod modules;

use components::footer::Footer;
use components::home::Homepage;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Homepage {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Pill Detect" }
        document::Link { rel: "icon", href: asset!("/assets/logo.svg") }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        document::Link { rel: "stylesheet", href: asset!("/assets/components/home.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/components/registration.css") }

        Router::<Route> {}
        Footer {}
    }
}
