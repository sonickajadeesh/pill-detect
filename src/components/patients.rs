use dioxus::prelude::*;

use crate::modules::{
    patient_db::{Patient, delete_patient},
    utilities::calculate_age,
};

#[component]
pub fn PatientList(on_edit: EventHandler<Patient>) -> Element {
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
            p { class: "mb-[30px] rounded-lg border border-gray-200 px-5 py-5 text-center text-sm text-gray-500",
                "No patients registered yet."
            }
        } else {
            div { class: "mb-[30px] w-full overflow-x-auto rounded-lg border border-gray-200",

                table { class: "w-full min-w-[500px] table-fixed border-collapse bg-white text-left",

                    colgroup {
                        col {}
                        col { class: "w-[50px]" }
                        col { class: "w-[70px]" }
                        col { class: "w-32" }
                        col { class: "w-[90px]" }
                    }

                    thead { class: "bg-gray-50",

                        tr {
                            th { class: "border-b border-gray-200 px-4 py-3 text-xs font-semibold uppercase tracking-[0.03em] text-gray-500",
                                "Patient name"
                            }
                            th { class: "border-b border-gray-200 px-4 py-3 text-xs font-semibold uppercase tracking-[0.03em] text-gray-500",
                                "Age"
                            }
                            th { class: "border-b border-gray-200 px-4 py-3 text-xs font-semibold uppercase tracking-[0.03em] text-gray-500",
                                "Sex"
                            }
                            th { class: "border-b border-gray-200 px-4 py-3 text-xs font-semibold uppercase tracking-[0.03em] text-gray-500",
                                "Blood group"
                            }
                            th { class: "border-b border-gray-200 px-4 py-3 text-xs font-semibold uppercase tracking-[0.03em] text-gray-500",
                                "Actions"
                            }
                        }
                    }

                    tbody {
                        for patient in patients.read().iter() {
                            tr {
                                key: "{patient.id}",
                                class: "hover:bg-gray-50",

                                td { class: "max-w-0 overflow-hidden text-ellipsis whitespace-nowrap border-b border-gray-100 px-4 py-3.5 text-sm font-medium text-gray-900",
                                    "{patient.first_name} {patient.last_name}"
                                }

                                td { class: "border-b border-gray-100 px-4 py-3.5 text-sm text-gray-700",

                                    match calculate_age(&patient.date_of_birth) {
                                        Some(age) => rsx! { "{age}" },
                                        None => rsx! { "—" },
                                    }
                                }

                                td { class: "border-b border-gray-100 px-4 py-3.5 text-sm capitalize text-gray-700",
                                    "{patient.sex}"
                                }

                                td { class: "border-b border-gray-100 px-4 py-3.5 text-sm text-gray-700",
                                    "{patient.blood_group}"
                                }

                                td { class: "border-b border-gray-100 px-4 py-3.5",

                                    div { class: "flex items-center gap-3",

                                        button {
                                            class: "cursor-pointer pt-0.5 text-base font-bold text-gray-600 hover:text-blue-500",
                                            r#type: "button",

                                            onclick: {
                                                let patient = patient.clone();

                                                move |_| {
                                                    on_edit.call(patient.clone());
                                                }
                                            },

                                            "🖉"
                                        }

                                        button {
                                            class: "cursor-pointer text-[25px] leading-none text-gray-600 hover:text-red-500",
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
