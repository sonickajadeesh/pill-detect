mod components;
mod modules;

use components::footer::Footer;
use components::registeration::RegistrationForm;
use dioxus::prelude::*;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/register")]
    RegistrationForm {},
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
        document::Link { rel: "stylesheet", href: asset!("/assets/home.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/registration.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        Router::<Route> {}
        Footer {}
    }
}

#[component]
pub fn Home() -> Element {
    let mut show_registration = use_signal(|| false);

    rsx! {
        div {
            class: "home-page",

            h1 {
                class: "project-name",
                "Pill Detect"
            }

            p {
                class: "project-subheading",
                "Prescription Analysis • Medicine Details • Symptom Guidance • Drug Interaction"
            }

            button {
                class: "register-button",
                r#type: "button",
                onclick: move |_| show_registration.set(true),
                "Register New Patient"
            }

            // Registration modal
            if show_registration() {
                div {
                    class: "modal-overlay",

                    // Clicking outside the modal closes it
                    onclick: move |_| show_registration.set(false),

                    div {
                        class: "modal-card",

                        // Prevent clicks inside the modal from closing it
                        onclick: move |event| event.stop_propagation(),

                        button {
                            class: "modal-close",
                            r#type: "button",
                            onclick: move |_| show_registration.set(false),
                            "×"
                        }

                        RegistrationForm {}
                    }
                }
            }
        }
    }
}
