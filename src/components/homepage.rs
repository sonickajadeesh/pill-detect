use crate::components::patients::PatientList;
use crate::components::registeration::RegistrationForm;
use crate::modules::patient_db::get_patients;
use dioxus::prelude::*;

#[component]
pub fn Homepage() -> Element {
    let mut show_registration = use_signal(|| false);

    let patients = use_signal(|| get_patients().unwrap_or_default());

    use_context_provider(|| patients);

    rsx! {
        div { class: "home-page",

            h1 { class: "project-name",  "🩺 Pill Detect" }

            p { class: "project-subheading",
                "Prescription Analysis • Medicine Details • Symptom Guidance • Drug Interaction"
            }

            PatientList {}

            button {
                class: "register-button",
                r#type: "button",
                onclick: move |_| show_registration.set(true),
                "Register New Patient"
            }

            if show_registration() {
                div {
                    class: "modal-overlay",

                    onclick: move |_| show_registration.set(false),

                    div {
                        class: "modal-card",

                        onclick: move |event| event.stop_propagation(),

                        RegistrationForm { on_close: move |_| show_registration.set(false) }
                    }
                }
            }
        }
    }
}
