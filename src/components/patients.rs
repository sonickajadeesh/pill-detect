use crate::modules::patient_db::{delete_patient, Patient};
use dioxus::prelude::*;

#[component]
pub fn PatientList() -> Element {
    let mut patients = use_context::<Signal<Vec<Patient>>>();

    let mut delete = move |patient_id: String| {
        if let Err(err) = delete_patient(&patient_id) {
            println!("Failed to delete patient: {}", err);
            return;
        }

        patients.write().retain(|patient| patient.id != patient_id);
    };

    rsx! {
        div { class: "patient-list",

            if patients.read().is_empty() {
                div { class: "no-patients", "No patients registered yet." }
            } else {
                div { class: "patient-table-wrapper",

                    table { class: "patient-table",

                        thead {
                            tr {
                                th { "Patient name" }
                                th { "Date of birth" }
                                th { "Sex" }
                                th { "Actions" }
                            }
                        }

                        tbody {
                            for patient in patients.read().iter() {
                                tr {
                                    td { class: "patient-name",
                                        "{patient.first_name} {patient.last_name}"
                                    }

                                    td { "{patient.date_of_birth}" }

                                    td { class: "patient-sex", "{patient.sex}" }

                                    td {
                                        button {
                                            class: "delete-button",
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
