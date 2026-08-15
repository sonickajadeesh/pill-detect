use crate::modules::Patient;
use dioxus::prelude::*;

#[component]
pub fn RegistrationForm(on_close: EventHandler<()>) -> Element {
    let mut first_name = use_signal(String::new);
    let mut last_name = use_signal(String::new);
    let mut sex = use_signal(String::new);
    let mut date_of_birth = use_signal(String::new);
    let mut blood_group = use_signal(String::new);
    let mut height = use_signal(String::new);
    let mut weight = use_signal(String::new);
    let mut allergies = use_signal(String::new);
    let mut medical_conditions = use_signal(String::new);

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
            id: uuid::Uuid::new_v4().to_string(),
            first_name: first_name().trim().to_string(),
            last_name: last_name().trim().to_string(),
            sex: sex(),
            date_of_birth: date_of_birth(),
            blood_group: blood_group(),
            height: height().parse::<u32>().unwrap_or(0),
            weight: weight().parse::<f32>().unwrap_or(0.0),
            allergies: allergies().trim().to_string(),
            medical_conditions: medical_conditions().trim().to_string(),
        };

        println!("{:#?}", patient);

        success.set("Patient registered successfully!".to_string());
    };

    rsx! {
        div { class: "register-page",

            div { class: "register-card",

                div { class: "register-header",

                    div {
                        h1 { "Patient Registration" }
                        p { "Enter the patient's medical information." }
                    }

                    button {
                        class: "modal-close",
                        r#type: "button",
                        onclick: move |_| on_close.call(()),
                        "×"
                    }
                }

                form { class: "register-form", onsubmit: submit,

                    // First name + Last name
                    div { class: "form-row",

                        div { class: "form-group",
                            label { r#for: "first-name", "First name" }

                            input {
                                id: "first-name",
                                r#type: "text",
                                placeholder: "Sonicka",
                                value: "{first_name}",
                                oninput: move |event| first_name.set(event.value()),
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "last-name", "Last name" }

                            input {
                                id: "last-name",
                                r#type: "text",
                                placeholder: "Jadeesh",
                                value: "{last_name}",
                                oninput: move |event| last_name.set(event.value()),
                            }
                        }
                    }

                    // Sex + Blood group
                    div { class: "form-row",

                        div { class: "form-group",
                            label { r#for: "sex", "Sex" }

                            select {
                                id: "sex",
                                value: "{sex}",
                                onchange: move |event| sex.set(event.value()),

                                option { value: "", disabled: true, "Select sex" }

                                option { value: "male", "Male" }

                                option { value: "female", "Female" }
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "blood-group", "Blood group" }

                            select {
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

                    // Date of birth
                    div { class: "form-group",

                        label { r#for: "date-of-birth", "Date of birth" }

                        input {
                            id: "date-of-birth",
                            r#type: "date",
                            value: "{date_of_birth}",
                            oninput: move |event| { date_of_birth.set(event.value()) },
                        }
                    }

                    // Height + Weight
                    div { class: "form-row",

                        div { class: "form-group",
                            label { r#for: "height", "Height" }

                            div { class: "input-with-unit",

                                input {
                                    id: "height",
                                    r#type: "number",
                                    min: "0",
                                    step: "1",
                                    placeholder: "160",
                                    value: "{height}",
                                    oninput: move |event| height.set(event.value()),
                                }

                                span { class: "input-unit", "cm" }
                            }
                        }

                        div { class: "form-group",
                            label { r#for: "weight", "Weight" }

                            div { class: "input-with-unit",

                                input {
                                    id: "weight",
                                    r#type: "number",
                                    min: "0",
                                    step: "0.1",
                                    placeholder: "55",
                                    value: "{weight}",
                                    oninput: move |event| weight.set(event.value()),
                                }

                                span { class: "input-unit", "kg" }
                            }
                        }
                    }

                    // Allergies
                    div { class: "form-group",

                        label { r#for: "allergies",
                            "Allergies"

                            span { class: "optional", " (if any)" }
                        }

                        textarea {
                            id: "allergies",
                            placeholder: "e.g. Penicillin, peanuts...",
                            value: "{allergies}",
                            oninput: move |event| allergies.set(event.value()),
                        }
                    }

                    // Medical conditions
                    div { class: "form-group",

                        label { r#for: "medical-conditions",
                            "Existing medical conditions"

                            span { class: "optional", " (if any)" }
                        }

                        textarea {
                            id: "medical-conditions",
                            placeholder: "e.g. Diabetes, hypertension...",
                            value: "{medical_conditions}",
                            oninput: move |event| { medical_conditions.set(event.value()) },
                        }
                    }

                    // Success
                    if !success().is_empty() {
                        div { class: "form-success", "{success}" }
                    }

                    // Error
                    if !error().is_empty() {
                        div { class: "form-error", "{error}" }
                    }

                    button { class: "register-button", r#type: "submit", "Register Patient" }
                }
            }
        }
    }
}
