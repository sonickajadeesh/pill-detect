use dioxus::prelude::*;

use crate::{Route, modules::api::clear_api_key};

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

            p { class: "mb-7 mt-2 max-w-xl text-sm text-gray-500 sm:mb-9 sm:text-base md:text-lg",
                "Prescription Analysis • Medicine Details • Symptom Guidance • Drug Interaction"
            }

            div { class: "w-full max-w-[700px] rounded-xl border border-gray-200 bg-white p-4 shadow-sm sm:p-6",

                div { class: "mb-5 sm:mb-6",

                    h2 { class: "text-xl font-bold text-gray-800 sm:text-2xl",
                        "{patient.first_name} {patient.last_name}"
                    }

                    p { class: "mt-1 text-sm text-gray-500", "What would you like to do?" }
                }

                div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4",

                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-4 py-5 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50 active:bg-blue-50 sm:px-5 sm:py-6",
                        r#type: "button",

                        "📋"

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Prescription"
                        }

                        p { class: "mt-1 text-xs text-gray-500 sm:text-sm",
                            "View and manage prescriptions"
                        }
                    }

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

                        "🔎"

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Medicine Details"
                        }

                        p { class: "mt-1 text-xs text-gray-500 sm:text-sm",
                            "View medicine information"
                        }
                    }

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

                        "💬"

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Symptom Guidance"
                        }

                        p { class: "mt-1 text-xs text-gray-500 sm:text-sm",
                            "Get guidance based on symptoms"
                        }
                    }

                    button {
                        class: "rounded-lg border border-gray-200 bg-white px-4 py-5 text-left shadow-sm transition hover:border-blue-400 hover:bg-blue-50 active:bg-blue-50 sm:px-5 sm:py-6",
                        r#type: "button",

                        "⚠️"

                        div { class: "mt-2 text-sm font-semibold text-gray-800 sm:text-base",
                            "Drug Interactions"
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
