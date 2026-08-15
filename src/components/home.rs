use crate::components::registeration::RegistrationForm;
use dioxus::prelude::*;

#[component]
pub fn Homepage() -> Element {
    let mut show_registration = use_signal(|| false);

    rsx! {
        div { class: "home-page",

            h1 { class: "project-name", "Pill Detect" }

            p { class: "project-subheading",
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

                        RegistrationForm { on_close: move |_| show_registration.set(false) }
                    }
                }
            }
        }
    }
}
