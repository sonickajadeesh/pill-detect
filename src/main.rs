mod components;
mod modules;

use components::footer::Footer;
use components::homepage::Homepage;
use components::information::Information;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Homepage {},

    #[route("/information")]
    Information {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_effect(|| {
        spawn(async {
            crate::modules::api::get_api_key().await.ok();
        });
    });
    rsx! {
        document::Title { "Pill Detect" }
        document::Link {
            rel: "icon",
            href: "data:image/svg+xml,
          <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
            <text y='0.9em' font-size='90'>🩺</text>
          </svg>",
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/components/homepage.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/components/information.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/components/patients.css"),
        }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/components/registration.css"),
        }

        // Router::<Route> {}
        Information {}
        Footer {}
    }
}
