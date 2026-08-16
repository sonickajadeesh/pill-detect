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
        div { class: "patient-list",

            if patients.read().is_empty() {
                div { class: "no-patients", "No patients registered yet." }
            } else {
                div { class: "patient-table-wrapper",

                    table { class: "patient-table",

                        colgroup {
                            col { class: "patient-name-col" }
                            col { class: "patient-age-col" }
                            col { class: "patient-sex-col" }
                            col { class: "patient-blood-group-col" }
                            col { class: "patient-actions-col" }
                        }

                        thead {
                            tr {
                                th { "Patient name" }
                                th { "Age" }
                                th { "Sex" }
                                th { "Blood group" }
                                th { "Actions" }
                            }
                        }

                        tbody {
                            for patient in patients.read().iter() {
                                tr { key: "{patient.id}",

                                    td { class: "patient-name",
                                        "{patient.first_name} {patient.last_name}"
                                    }

                                    td {
                                        match calculate_age(&patient.date_of_birth) {
                                            Some(age) => rsx! { "{age}" },
                                            None => rsx! { "—" },
                                        }
                                    }

                                    td { class: "patient-sex", "{patient.sex}" }

                                    td { "{patient.blood_group}" }

                                    td { class: "patient-actions",

                                        // Edit
                                        button {
                                            class: "edit-button",
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
