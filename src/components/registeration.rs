use dioxus::prelude::*;

use crate::modules::{
    patient_db::{Patient, add_patient, update_patient},
    utilities::{is_valid_date, sentence_case},
};

#[component]
pub fn RegistrationForm(on_close: EventHandler<()>, patient: Option<Patient>) -> Element {
    let mut patients = use_context::<Signal<Vec<Patient>>>();

    let is_editing = patient.is_some();

    let mut first_name = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.first_name.clone())
            .unwrap_or_default()
    });

    let mut last_name = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.last_name.clone())
            .unwrap_or_default()
    });

    let mut sex = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.sex.clone())
            .unwrap_or_default()
    });

    let mut date_of_birth = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.date_of_birth.clone())
            .unwrap_or_default()
    });

    let mut blood_group = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.blood_group.clone())
            .unwrap_or_default()
    });

    let mut height = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.height.to_string())
            .unwrap_or_default()
    });

    let mut weight = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.weight.to_string())
            .unwrap_or_default()
    });

    let mut allergies = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.allergies.clone())
            .unwrap_or_default()
    });

    let mut medical_conditions = use_signal(|| {
        patient
            .as_ref()
            .map(|patient| patient.medical_conditions.clone())
            .unwrap_or_default()
    });

    let mut error = use_signal(String::new);
    let mut success = use_signal(String::new);

    let submit = move |event: FormEvent| {
        event.prevent_default();

        error.set(String::new());
        success.set(String::new());

        if first_name().trim().is_empty() {
            error.set("Please enter the patient's first name.".to_string());
            return;
        }

        if last_name().trim().is_empty() {
            error.set("Please enter the patient's last name.".to_string());
            return;
        }

        if sex().is_empty() {
            error.set("Please select the patient's sex.".to_string());
            return;
        }

        if date_of_birth().is_empty() {
            error.set("Please enter the patient's date of birth.".to_string());
            return;
        }

        if !is_valid_date(&date_of_birth()) {
            error.set("Date of birth cannot be in the future.".to_string());
            return;
        }

        if blood_group().is_empty() {
            error.set("Please select the patient's blood group.".to_string());
            return;
        }

        if height().trim().is_empty() {
            error.set("Please enter the patient's height.".to_string());
            return;
        }

        if weight().trim().is_empty() {
            error.set("Please enter the patient's weight.".to_string());
            return;
        }

        let patient = Patient {
            id: patient
                .as_ref()
                .map(|patient| patient.id.clone())
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            first_name: sentence_case(&first_name()),
            last_name: sentence_case(&last_name()),
            sex: sex(),
            date_of_birth: date_of_birth(),
            blood_group: blood_group(),
            height: height().parse::<u32>().unwrap_or(0),
            weight: weight().parse::<f32>().unwrap_or(0.0),
            allergies: allergies().trim().to_string(),
            medical_conditions: medical_conditions().trim().to_string(),
        };

        if is_editing {
            match update_patient(patient.clone()) {
                Ok(_) => {
                    if let Some(existing_patient) = patients
                        .write()
                        .iter_mut()
                        .find(|existing| existing.id == patient.id)
                    {
                        *existing_patient = patient;
                    }

                    success.set("Patient updated successfully!".to_string());
                }
                Err(err) => error.set(err),
            }
        } else {
            match add_patient(patient.clone()) {
                Ok(_) => {
                    patients.write().push(patient);

                    first_name.set(String::new());
                    last_name.set(String::new());
                    sex.set(String::new());
                    date_of_birth.set(String::new());
                    blood_group.set(String::new());
                    height.set(String::new());
                    weight.set(String::new());
                    allergies.set(String::new());
                    medical_conditions.set(String::new());

                    success.set("Patient registered successfully!".to_string());
                }
                Err(err) => error.set(err),
            }
        }
    };

    rsx! {
        main { class: "bg-white p-5 text-left",

            header { class: "mb-5 flex items-start justify-between",

                div {
                    h1 { class: "mb-2 text-[28px] font-bold text-gray-800",
                        if is_editing {
                            "Edit Patient"
                        } else {
                            "Patient Registration"
                        }
                    }

                    p { class: "text-gray-500",
                        if is_editing {
                            "Update the patient's medical information."
                        } else {
                            "Enter the patient's medical information."
                        }
                    }
                }

                button {
                    class: "text-2xl leading-none text-gray-400 hover:text-gray-700",
                    r#type: "button",
                    onclick: move |_| on_close.call(()),
                    "×"
                }
            }

            form { class: "flex flex-col gap-[18px]", onsubmit: submit,

                div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",

                    label { class: "flex flex-col gap-1.5",

                        span { class: "text-sm font-semibold text-gray-700", "First name" }

                        input {
                            class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                            id: "first-name",
                            r#type: "text",
                            placeholder: "Sonicka",
                            value: "{first_name}",
                            oninput: move |event| first_name.set(event.value()),
                        }
                    }

                    label { class: "flex flex-col gap-1.5",

                        span { class: "text-sm font-semibold text-gray-700", "Last name" }

                        input {
                            class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                            id: "last-name",
                            r#type: "text",
                            placeholder: "Jadeesh",
                            value: "{last_name}",
                            oninput: move |event| last_name.set(event.value()),
                        }
                    }
                }

                div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",

                    label { class: "flex flex-col gap-1.5",

                        span { class: "text-sm font-semibold text-gray-700", "Sex" }

                        select {
                            class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                            id: "sex",
                            value: "{sex}",
                            onchange: move |event| sex.set(event.value()),

                            option { value: "", disabled: true, "Select sex" }
                            option { value: "male", "Male" }
                            option { value: "female", "Female" }
                        }
                    }

                    label { class: "flex flex-col gap-1.5",

                        span { class: "text-sm font-semibold text-gray-700", "Blood group" }

                        select {
                            class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                            id: "blood-group",
                            value: "{blood_group}",
                            onchange: move |event| blood_group.set(event.value()),

                            option { value: "", disabled: true, "Select blood group" }
                            option { value: "A+", "A+" }
                            option { value: "A-", "A−" }
                            option { value: "B+", "B+" }
                            option { value: "B-", "B−" }
                            option { value: "AB+", "AB+" }
                            option { value: "AB-", "AB−" }
                            option { value: "O+", "O+" }
                            option { value: "O-", "O−" }
                        }
                    }
                }

                label { class: "flex flex-col gap-1.5",

                    span { class: "text-sm font-semibold text-gray-700", "Date of birth" }

                    input {
                        class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                        id: "date-of-birth",
                        r#type: "date",
                        value: "{date_of_birth}",
                        oninput: move |event| date_of_birth.set(event.value()),
                    }
                }

                div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",

                    label { class: "flex flex-col gap-1.5",

                        span { class: "text-sm font-semibold text-gray-700", "Height" }

                        div { class: "flex items-center gap-2",

                            input {
                                class: "w-full min-w-0 rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                id: "height",
                                r#type: "number",
                                min: "0",
                                step: "1",
                                placeholder: "160",
                                value: "{height}",
                                oninput: move |event| height.set(event.value()),
                            }

                            span { class: "text-sm text-gray-500", "cm" }
                        }
                    }

                    label { class: "flex flex-col gap-1.5",

                        span { class: "text-sm font-semibold text-gray-700", "Weight" }

                        div { class: "flex items-center gap-2",

                            input {
                                class: "w-full min-w-0 rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                id: "weight",
                                r#type: "number",
                                min: "0",
                                step: "0.1",
                                placeholder: "55",
                                value: "{weight}",
                                oninput: move |event| weight.set(event.value()),
                            }

                            span { class: "text-sm text-gray-500", "kg" }
                        }
                    }
                }

                label { class: "flex flex-col gap-1.5",

                    span { class: "text-sm font-semibold text-gray-700",
                        "Allergies"
                        span { class: "font-normal text-gray-400", " (if any)" }
                    }

                    textarea {
                        class: "min-h-[90px] w-full resize-y rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                        id: "allergies",
                        placeholder: "e.g. Penicillin, peanuts...",
                        value: "{allergies}",
                        oninput: move |event| allergies.set(event.value()),
                    }
                }

                label { class: "flex flex-col gap-1.5",

                    span { class: "text-sm font-semibold text-gray-700",
                        "Existing medical conditions"
                        span { class: "font-normal text-gray-400", " (if any)" }
                    }

                    textarea {
                        class: "min-h-[90px] w-full resize-y rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                        id: "medical-conditions",
                        placeholder: "e.g. Diabetes, hypertension...",
                        value: "{medical_conditions}",
                        oninput: move |event| medical_conditions.set(event.value()),
                    }
                }

                if !success().is_empty() {
                    div { class: "rounded-md border border-green-200 bg-green-50 px-3 py-2.5 text-sm text-green-700",
                        "{success}"
                    }
                }

                if !error().is_empty() {
                    div { class: "rounded-md border border-red-200 bg-red-50 px-3 py-2.5 text-sm text-red-700",
                        "{error}"
                    }
                }

                button {
                    class: "w-full rounded-md bg-blue-600 px-[18px] py-3 text-[15px] font-semibold text-white hover:bg-blue-700 active:bg-blue-800",
                    r#type: "submit",

                    if is_editing {
                        "Update Patient"
                    } else {
                        "Register Patient"
                    }
                }
            }
        }
    }
}
