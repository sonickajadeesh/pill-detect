use dioxus::prelude::*;

use crate::{
    Route,
    modules::{
        api::clear_api_key,
        utilities::{calculate_age, sentence_case},
    },
};

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

    let age = calculate_age(&patient.date_of_birth).unwrap_or(0);

    rsx! {
        main { class: "relative flex min-h-[96vh] flex-col items-center px-4 py-10 text-center sm:px-5 sm:py-12",

            // Back button
            Link {
                to: Route::Homepage {},
                class: "absolute left-4 top-5 flex items-center gap-1 text-sm font-medium text-slate-600 transition-colors hover:text-slate-900 sm:left-5",

                "← Back"
            }

            // API key button
            button {
                class: "absolute right-4 top-5 flex h-10 w-10 items-center justify-center rounded-full border border-slate-200 bg-white text-lg shadow-sm transition hover:bg-slate-200 active:bg-slate-200 sm:right-5",
                r#type: "button",
                title: "Clear stored API key",

                onclick: move |_| {
                    spawn(async move {
                        if let Err(err) = clear_api_key().await {
                            eprintln!("Failed to change API key: {err}");
                        }
                    });
                },

                "🔑"
            }

            h1 { class: "m-0 mt-8 text-[40px] font-bold tracking-[-1.5px] text-gray-800 sm:mt-0 sm:text-[56px] md:text-[64px]",
                "🩺 Pill Detect"
            }

            div { class: "mt-6 w-full max-w-[700px] rounded-xl border border-gray-200 bg-white p-4 shadow-sm sm:p-6",

                div { class: "mb-5 sm:mb-6",

                    // Patient name
                    h2 { class: "text-xl font-bold text-gray-800 sm:text-2xl",
                        "{patient.first_name} {patient.last_name}"
                    }

                    // Patient details
                    div { class: "mt-4 border-t border-gray-100 pt-4 text-left",

                        // Basic information
                        div { class: "text-center grid grid-cols-2 gap-x-6 gap-y-3 sm:grid-cols-5",

                            div {
                                p { class: "text-xs font-medium uppercase tracking-wide text-gray-400",
                                    "Age"
                                }
                                p { class: "mt-0.5 text-sm font-semibold text-gray-700",
                                    "{age} years"
                                }
                            }

                            div {
                                p { class: "text-xs font-medium uppercase tracking-wide text-gray-400",
                                    "Sex"
                                }
                                p { class: "mt-0.5 text-sm font-semibold text-gray-700",
                                    "{sentence_case(&patient.sex)}"
                                }
                            }

                            div {
                                p { class: "text-xs font-medium uppercase tracking-wide text-gray-400",
                                    "Blood Group"
                                }
                                p { class: "mt-0.5 text-sm font-semibold text-gray-700",
                                    "{sentence_case(&patient.blood_group)}"
                                }
                            }

                            div {
                                p { class: "text-xs font-medium uppercase tracking-wide text-gray-400",
                                    "Height"
                                }
                                p { class: "mt-0.5 text-sm font-semibold text-gray-700",
                                    "{patient.height} cm"
                                }
                            }

                            div {
                                p { class: "text-xs font-medium uppercase tracking-wide text-gray-400",
                                    "Weight"
                                }
                                p { class: "mt-0.5 text-sm font-semibold text-gray-700",
                                    "{patient.weight} kg"
                                }
                            }
                        }

                        div { class: "mt-4 flex flex-wrap justify-center gap-2",

                            span { class: "rounded-full bg-amber-50 px-3 py-1.5 text-sm text-amber-900",
                                "Allergy: {sentence_case(&patient.allergies)}"
                            }

                            span { class: "rounded-full bg-slate-200 px-3 py-1.5 text-sm text-slate-700",
                                "Conditions: {sentence_case(&patient.medical_conditions)}"
                            }
                        }
                    }

                    p { class: "mt-5 text-sm text-gray-500", "What would you like to do?" }
                }

                div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4",

                    // Prescription Analysis
                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-4 py-5 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50 active:bg-blue-50 sm:px-5 sm:py-6",
                        r#type: "button",

                        onclick: {
                            let patient_id = patient_id.clone();

                            move |_| {
                                navigator
                                    .push(Route::PrescriptionAnalysis {
                                        patient_id: patient_id.clone(),
                                    });
                            }
                        },

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Prescription Analysis 📋"
                        }

                        p { class: "mt-1 text-xs text-gray-500 sm:text-sm",
                            "View and manage prescriptions"
                        }
                    }

                    // Medicine Details
                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-4 py-5 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50 active:bg-blue-50 sm:px-5 sm:py-6",

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

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Medicine Details 🔎"
                        }

                        p { class: "mt-1 text-xs text-gray-500 sm:text-sm",
                            "View medicine information"
                        }
                    }

                    // Symptom Guidance
                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-4 py-5 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50 active:bg-blue-50 sm:px-5 sm:py-6",

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

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Symptom Guidance 💬"
                        }

                        p { class: "mt-1 text-xs text-gray-500 sm:text-sm",
                            "Get guidance based on symptoms"
                        }
                    }

                    // Drug Interactions
                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-4 py-5 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50 active:bg-blue-50 sm:px-5 sm:py-6",

                        r#type: "button",

                        onclick: {
                            let patient_id = patient_id.clone();

                            move |_| {
                                navigator
                                    .push(Route::DrugInteraction {
                                        patient_id: patient_id.clone(),
                                    });
                            }
                        },

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Drug Interactions ⚠️"
                        }

                        p { class: "mt-1 text-xs text-gray-500 sm:text-sm",
                            "Check potential drug interactions"
                        }
                    }
                }
            }
        }
    }
}
