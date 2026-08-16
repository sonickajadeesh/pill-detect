use crate::modules::patient_db::Patient;
use dioxus::prelude::*;

#[component]
pub fn PatientList() -> Element {
    let patients = use_context::<Signal<Vec<Patient>>>();

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
                            }
                        }

                        tbody {
                            for patient in patients.read().iter() {
                                tr { key: "{patient.id}",

                                    td { class: "patient-name",
                                        "{patient.first_name} {patient.last_name}"
                                    }

                                    td { "{patient.date_of_birth}" }

                                    td { class: "patient-sex", "{patient.sex}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
