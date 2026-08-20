use dioxus::prelude::*;

use crate::{Route, components::navbar::Navbar};

#[component]
pub fn Dashboard(patient_id: String) -> Element {
    let navigator = use_navigator();

    // Load patient using patient_id
    let patient = match crate::modules::database::get_patient_id(&patient_id) {
        Ok(patient) => patient,
        Err(err) => {
            eprintln!("Failed to load patient: {err}");
            return rsx! {
                p { "Patient not found." }
            };
        }
    };

    rsx! {
        Navbar { patient_id: patient_id.clone() }

        main { class: "relative flex min-h-[90vh] flex-col items-center justify-center px-5 py-10 text-center",
            div { class: "w-[90%] max-w-[700px] rounded-xl border border-gray-200 bg-white p-6 shadow-sm",

                div { class: "mb-6",

                    h2 { class: "text-2xl font-bold text-gray-800",
                        "{patient.first_name} {patient.last_name}"
                    }

                    p { class: "mt-1 text-sm text-gray-500", "What would you like to do?" }
                }

                div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",

                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-5 py-6 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50",
                        r#type: "button",

                        "📋"
                        div { class: "mt-2 text-base font-semibold text-gray-800", "Prescription" }
                        p { class: "mt-1 text-sm text-gray-500", "View and manage prescriptions" }
                    }

                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-5 py-6 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50",
                        r#type: "button",

                        onclick: {
                            let patient_id = patient_id.clone();

                            move |_| {
                                navigator
                                    .push(Route::Information {
                                        patient_id: patient_id.clone(),
                                    });
                            }
                        },

                        "💊"
                        div { class: "mt-2 text-base font-semibold text-gray-800", "Medicine Details" }
                        p { class: "mt-1 text-sm text-gray-500", "View medicine information" }
                    }

                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-5 py-6 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50",
                        r#type: "button",

                        onclick: {
                            let patient_id = patient_id.clone();

                            move |_| {
                                navigator
                                    .push(Route::Guidance {
                                        patient_id: patient_id.clone(),
                                    });
                            }
                        },

                        "🩺"
                        div { class: "mt-2 text-base font-semibold text-gray-800", "Symptom Guidance" }
                        p { class: "mt-1 text-sm text-gray-500", "Get guidance based on symptoms" }
                    }

                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-5 py-6 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50",
                        r#type: "button",

                        "⚠️"
                        div { class: "mt-2 text-base font-semibold text-gray-800", "Drug Interactions" }
                        p { class: "mt-1 text-sm text-gray-500", "Check potential drug interactions" }
                    }
                }
            }
        }
    }
}
