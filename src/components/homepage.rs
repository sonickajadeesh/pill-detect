use crate::components::patients::PatientList;
use crate::components::registeration::RegistrationForm;
use crate::modules::patient_db::{Patient, get_patients};
use dioxus::prelude::*;

#[component]
pub fn Homepage() -> Element {
    let mut show_registration = use_signal(|| false);

    // None = registering a new patient
    // <patient> = editing an existing patient
    let mut editing_patient = use_signal(|| None::<Patient>);

    // Shared reactive patient list
    let patients = use_signal(|| get_patients().unwrap_or_default());

    use_context_provider(|| patients);

    rsx! {
        div { class: "home-page",

            h1 { class: "project-name", "🩺 Pill Detect" }

            p { class: "project-subheading",
                "Prescription Analysis • Medicine Details • Symptom Guidance • Drug Interaction"
            }

            PatientList {
                on_edit: move |patient| {
                    editing_patient.set(Some(patient));
                    show_registration.set(true);
                },
            }

            button {
                class: "register-button",
                r#type: "button",

                onclick: move |_| {
                    // New registration
                    editing_patient.set(None);
                    show_registration.set(true);
                },

                "Register New Patient"
            }

            // Registration / Edit modal
            if show_registration() {
                div {
                    class: "modal-overlay",

                    // Clicking outside the modal closes it
                    onclick: move |_| {
                        show_registration.set(false);
                        editing_patient.set(None);
                    },

                    div {
                        class: "modal-card",

                        // Prevent clicks inside the modal from closing it
                        onclick: move |event| event.stop_propagation(),

                        RegistrationForm {
                            patient: editing_patient(),
                            on_close: move |_| {
                                show_registration.set(false);
                                editing_patient.set(None);
                            },
                        }
                    }
                }
            }
        }
    }
}
