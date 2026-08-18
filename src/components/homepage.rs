use dioxus::prelude::*;

use crate::Route;
use crate::components::{patients::PatientList, registeration::RegistrationForm};
use crate::modules::database::{Patient, get_patients};

#[component]
pub fn Homepage() -> Element {
    let mut show_registration = use_signal(|| false);
    let mut editing_patient = use_signal(|| None::<Patient>);

    let patients = use_signal(|| get_patients().unwrap_or_default());

    use_context_provider(|| patients);

    let navigator = use_navigator();

    rsx! {
        main { class: "flex min-h-[96vh] flex-col items-center justify-center px-5 py-10 text-center",

            h1 { class: "m-0 text-[44px] font-bold tracking-[-2px] text-gray-800 sm:text-[64px]",
                "🩺 Pill Detect"
            }

            p { class: "mb-9 mt-2 text-base text-gray-500 sm:text-lg",
                "Prescription Analysis • Medicine Details • Symptom Guidance • Drug Interaction"
            }

            PatientList {
                on_edit: move |patient: Patient| {
                    editing_patient.set(Some(patient));
                    show_registration.set(true);
                },

                on_select: move |patient: Patient| {
                    navigator
                        .push(Route::Information {
                            patient_id: patient.id,
                        });
                },
            }

            button {
                class: "min-w-[220px] rounded-md bg-blue-600 px-5 py-3 text-sm font-semibold text-white hover:bg-blue-700 active:bg-blue-800",
                r#type: "button",

                onclick: move |_| {
                    editing_patient.set(None);
                    show_registration.set(true);
                },

                "Register New Patient"
            }

            if show_registration() {
                div {
                    class: "fixed inset-0 z-[1000] flex items-center justify-center bg-black/50 p-5",

                    onclick: move |_| {
                        show_registration.set(false);
                        editing_patient.set(None);
                    },

                    div {
                        class: "relative max-h-[90vh] w-full max-w-[600px] overflow-y-auto rounded-xl bg-white shadow-[0_10px_40px_rgba(0,0,0,0.2)]",

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
