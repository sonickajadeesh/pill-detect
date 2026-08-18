use dioxus::prelude::*;

use crate::modules::{
    database::{Patient, delete_patient},
    utilities::calculate_age,
};

#[component]
pub fn PatientList(on_edit: EventHandler<Patient>, on_select: EventHandler<Patient>) -> Element {
    let mut patients = use_context::<Signal<Vec<Patient>>>();

    let mut delete = move |patient_id: String| {
        if let Err(err) = delete_patient(&patient_id) {
            println!("Failed to delete patient: {}", err);
            return;
        }

        patients.write().retain(|patient| patient.id != patient_id);
    };

    rsx! {
        if patients.read().is_empty() {
            p { class: "mb-[30px] w-[90%] rounded-lg border border-gray-200 px-5 py-5 text-center text-sm text-gray-500 sm:w-1/2",
                "No patients registered yet."
            }
        } else {
            div { class: "mb-[30px] w-[90%] sm:w-1/2",

                div { class: "w-full overflow-x-auto rounded-lg border border-gray-200",

                    table { class: "w-full table-fixed border-collapse bg-white text-left",

                        colgroup {
                            col {}
                            col { class: "w-[40px] sm:w-[50px]" }
                            col { class: "w-[55px] sm:w-[70px]" }
                            col { class: "w-[65px] sm:w-32" }
                            col { class: "w-[65px] sm:w-[90px]" }
                        }

                        thead { class: "bg-gray-50",

                            tr {
                                th { class: "border-b border-gray-200 px-2 py-2 text-[10px] font-semibold uppercase tracking-wide text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                    "Patient Name"
                                }

                                th { class: "border-b border-gray-200 px-1 py-2 text-[10px] font-semibold uppercase tracking-wide text-center text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                    "Age"
                                }

                                th { class: "border-b border-gray-200 px-1 py-2 text-[10px] font-semibold uppercase tracking-wide text-center text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                    "Sex"
                                }

                                th { class: "border-b border-gray-200 px-1 py-2 text-[10px] font-semibold uppercase tracking-wide text-center text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                    "Blood Group"
                                }

                                th { class: "border-b border-gray-200 px-1 py-2 text-[10px] font-semibold uppercase tracking-wide text-center text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                    "Actions"
                                }
                            }
                        }

                        tbody {
                            for patient in patients.read().iter() {
                                tr {
                                    key: "{patient.id}",
                                    class: "hover:bg-gray-50",

                                    td { class: "max-w-0 overflow-hidden text-ellipsis whitespace-nowrap border-b border-gray-100 px-2 py-3 text-xs font-medium text-gray-900 sm:px-4 sm:py-3.5 sm:text-sm",

                                        button {
                                            class: "cursor-pointer hover:text-blue-600 hover:underline",
                                            r#type: "button",

                                            onclick: {
                                                let patient = patient.clone();

                                                move |_| {
                                                    on_select.call(patient.clone());
                                                }
                                            },

                                            "{patient.first_name} {patient.last_name}"
                                        }
                                    }

                                    td { class: "border-b border-gray-100 px-1 py-3 text-center text-xs text-gray-700 sm:px-4 sm:py-3.5 sm:text-sm",

                                        match calculate_age(&patient.date_of_birth) {
                                            Some(age) => rsx! { "{age}" },
                                            None => rsx! { "—" },
                                        }
                                    }

                                    td { class: "border-b border-gray-100 px-1 py-3 text-xs capitalize text-gray-700 sm:px-4 sm:py-3.5 sm:text-sm",
                                        "{patient.sex}"
                                    }

                                    td { class: "border-b border-gray-100 px-1 py-3 text-center text-xs text-gray-700 sm:px-4 sm:py-3.5 sm:text-sm",
                                        "{patient.blood_group}"
                                    }

                                    td { class: "border-b border-gray-100 px-1 py-3 sm:px-4 sm:py-3.5",

                                        div { class: "flex items-center justify-center gap-3",

                                            // Edit
                                            button {
                                                class: "cursor-pointer text-sm font-bold text-gray-600 hover:text-blue-500 sm:text-base",
                                                r#type: "button",

                                                onclick: {
                                                    let patient = patient.clone();

                                                    move |_| {
                                                        on_edit.call(patient.clone());
                                                    }
                                                },

                                                "🖉"
                                            }

                                            // Delete
                                            button {
                                                class: "cursor-pointer text-xl leading-none text-gray-600 hover:text-red-500 sm:text-[25px]",
                                                r#type: "button",

                                                onclick: {
                                                    let patient_id = patient.id.clone();

                                                    move |_| {
                                                        if web_sys::window()
                                                            .and_then(|window| {
                                                                window
                                                                    .confirm_with_message(
                                                                        "Are you sure you want to delete this patient?",
                                                                    )
                                                                    .ok()
                                                            })
                                                            .unwrap_or(false)
                                                        {
                                                            delete(patient_id.clone());
                                                        }
                                                    }
                                                },

                                                "⨯"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
