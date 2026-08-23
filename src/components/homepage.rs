use dioxus::prelude::*;

use crate::{
    Route,
    modules::{
        api::clear_api_key,
        database::{Patient, add_patient, delete_patient, get_patients, update_patient},
        utilities::{calculate_age, sentence_case, validate},
    },
};

#[component]
pub fn Homepage() -> Element {
    let mut show_registration = use_signal(|| false);
    let mut editing_patient = use_signal(|| None::<Patient>);
    let mut patients = use_signal(|| get_patients().unwrap_or_default());

    // Registration form state
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
    let mut submitted = use_signal(|| false);

    let navigator = use_navigator();

    let mut refresh_patients = move || {
        patients.set(get_patients().unwrap_or_default());
    };

    let mut reset_form = move || {
        first_name.set(String::new());
        last_name.set(String::new());
        sex.set(String::new());
        date_of_birth.set(String::new());
        blood_group.set(String::new());
        height.set(String::new());
        weight.set(String::new());
        allergies.set(String::new());
        medical_conditions.set(String::new());
        error.set(String::new());
        success.set(String::new());
        submitted.set(false);
    };

    let mut open_new_patient = move || {
        editing_patient.set(None);
        reset_form();
        show_registration.set(true);
    };

    let mut open_edit_patient = move |patient: Patient| {
        first_name.set(patient.first_name.clone());
        last_name.set(patient.last_name.clone());
        sex.set(patient.sex.clone());
        date_of_birth.set(patient.date_of_birth.clone());
        blood_group.set(patient.blood_group.clone());
        height.set(patient.height.to_string());
        weight.set(patient.weight.to_string());
        allergies.set(patient.allergies.clone());
        medical_conditions.set(patient.medical_conditions.clone());

        error.set(String::new());
        success.set(String::new());
        submitted.set(false);

        editing_patient.set(Some(patient));
        show_registration.set(true);
    };

    let mut close_registration = move || {
        show_registration.set(false);
        editing_patient.set(None);
        reset_form();
    };

    let submit = move |event: FormEvent| {
        event.prevent_default();

        error.set(String::new());
        success.set(String::new());

        if let Some(message) = validate(
            &first_name(),
            &last_name(),
            &sex(),
            &date_of_birth(),
            &blood_group(),
            &height(),
            &weight(),
        ) {
            error.set(message);
            return;
        }

        let patient = Patient {
            id: editing_patient()
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

            chat_history: editing_patient()
                .as_ref()
                .map(|patient| patient.chat_history.clone())
                .unwrap_or_default(),

            interaction_history: editing_patient()
                .as_ref()
                .map(|patient| patient.interaction_history.clone())
                .unwrap_or_default(),

            prescriptions: editing_patient()
                .as_ref()
                .map(|patient| patient.prescriptions.clone())
                .unwrap_or_default(),

            search_history: editing_patient()
                .as_ref()
                .map(|patient| patient.search_history.clone())
                .unwrap_or_default(),
        };

        if editing_patient().is_some() {
            match update_patient(patient) {
                Ok(_) => {
                    success.set("Patient updated successfully!".to_string());
                    submitted.set(true);
                }

                Err(err) => {
                    error.set(err);
                }
            }
        } else {
            match add_patient(patient) {
                Ok(_) => {
                    success.set("Patient registered successfully!".to_string());
                    submitted.set(true);
                }

                Err(err) => {
                    error.set(err);
                }
            }
        }
    };

    let mut delete_patient_by_id = move |patient_id: String| {
        if let Err(err) = delete_patient(&patient_id) {
            eprintln!("Failed to delete patient: {err}");
            return;
        }

        patients.write().retain(|patient| patient.id != patient_id);
    };

    rsx! {
        main { class: "relative flex min-h-[96vh] flex-col items-center justify-center px-5 py-10 text-center",

            // Clear API key
            button {
                class: "absolute right-5 top-5 flex h-10 w-10 items-center justify-center rounded-full border border-slate-200 bg-white text-lg shadow-sm transition hover:bg-slate-200 active:bg-slate-200",
                r#type: "button",
                title: "Clear stored API key",

                onclick: move |_| {
                    spawn(async move {
                        if let Err(err) = clear_api_key().await {
                            eprintln!("Failed to clear API key: {err}");
                        }
                    });
                },

                "🔑"
            }

            // Header
            h1 { class: "m-0 text-[44px] font-bold tracking-[-2px] text-gray-800 sm:text-[64px]",
                "🩺 Pill Detect"
            }

            p { class: "mb-9 mt-2 text-base text-gray-500 sm:text-lg",
                "Prescription Analysis • Medicine Details • Symptom Guidance • Drug Interaction"
            }

            // Patient list
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

                                    th { class: "border-b border-gray-200 px-1 py-2 text-center text-[10px] font-semibold uppercase tracking-wide text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                        "Age"
                                    }

                                    th { class: "border-b border-gray-200 px-1 py-2 text-center text-[10px] font-semibold uppercase tracking-wide text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                        "Sex"
                                    }

                                    th { class: "border-b border-gray-200 px-1 py-2 text-center text-[10px] font-semibold uppercase tracking-wide text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
                                        "Blood Group"
                                    }

                                    th { class: "border-b border-gray-200 px-1 py-2 text-center text-[10px] font-semibold uppercase tracking-wide text-gray-500 sm:px-4 sm:py-3 sm:text-xs",
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
                                                    let patient_id = patient.id.clone();

                                                    move |_| {
                                                        navigator
                                                            .push(Route::Dashboard {
                                                                patient_id: patient_id.clone(),
                                                            });
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
                                                    title: "Edit patient",

                                                    onclick: {
                                                        let patient = patient.clone();

                                                        move |_| {
                                                            open_edit_patient(patient.clone());
                                                        }
                                                    },

                                                    "🖉"
                                                }

                                                // Delete
                                                button {
                                                    class: "cursor-pointer text-xl leading-none text-gray-600 hover:text-red-500 sm:text-[25px]",
                                                    r#type: "button",
                                                    title: "Delete patient",

                                                    onclick: {
                                                        let patient_id = patient.id.clone();

                                                        move |_| {
                                                            let confirmed = web_sys::window()
                                                                .and_then(|window| {
                                                                    window
                                                                        .confirm_with_message(
                                                                            "Are you sure you want to delete this patient?",
                                                                        )
                                                                        .ok()
                                                                })
                                                                .unwrap_or(false);

                                                            if confirmed {
                                                                delete_patient_by_id(patient_id.clone());
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

            // Register button
            button {
                class: "min-w-[220px] rounded-md bg-blue-600 px-5 py-3 text-sm font-semibold text-white hover:bg-blue-700 active:bg-blue-800",
                r#type: "button",

                onclick: move |_| {
                    open_new_patient();
                },

                "Register New Patient"
            }

            // Registration modal
            if show_registration() {
                div {
                    class: "fixed inset-0 z-[1000] flex items-center justify-center bg-black/50 p-5",

                    div {
                        class: "relative max-h-[90vh] w-full max-w-[600px] overflow-y-auto rounded-xl bg-white shadow-[0_10px_40px_rgba(0,0,0,0.2)]",

                        onclick: move |event| {
                            event.stop_propagation();
                        },

                        main { class: "bg-white p-5 text-left",

                            if submitted() {
                                div { class: "w-full",

                                    div { class: "w-full px-3 py-4 text-center", "{success}" }

                                    button {
                                        class: "mx-auto mt-3 block rounded-md bg-blue-600 px-6 py-2 text-sm font-semibold text-white hover:bg-blue-700",
                                        r#type: "button",

                                        onclick: move |_| {
                                            refresh_patients();
                                            close_registration();
                                        },

                                        "Okay"
                                    }
                                }
                            } else {
                                header { class: "mb-5 flex items-start justify-between",

                                    h1 { class: "text-[28px] font-bold text-gray-800",

                                        if editing_patient().is_some() {
                                            "Edit Patient"
                                        } else {
                                            "Patient Registration"
                                        }
                                    }

                                    button {
                                        class: "text-2xl leading-none text-gray-400 hover:text-gray-700",
                                        r#type: "button",

                                        onclick: move |_| {
                                            close_registration();
                                        },

                                        "×"
                                    }
                                }

                                form {
                                    class: "flex flex-col gap-[18px]",
                                    onsubmit: submit,

                                    div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",

                                        label { class: "flex flex-col gap-1.5",

                                            span { class: "text-sm font-semibold text-gray-700",
                                                "First name"
                                            }

                                            input {
                                                class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                                r#type: "text",
                                                placeholder: "Sonicka",
                                                value: "{first_name}",

                                                oninput: move |event| {
                                                    first_name.set(event.value());
                                                },
                                            }
                                        }

                                        label { class: "flex flex-col gap-1.5",

                                            span { class: "text-sm font-semibold text-gray-700",
                                                "Last name"
                                            }

                                            input {
                                                class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                                r#type: "text",
                                                placeholder: "Jadeesh",
                                                value: "{last_name}",

                                                oninput: move |event| {
                                                    last_name.set(event.value());
                                                },
                                            }
                                        }
                                    }

                                    div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",

                                        label { class: "flex flex-col gap-1.5",

                                            span { class: "text-sm font-semibold text-gray-700",
                                                "Sex"
                                            }

                                            select {
                                                class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                                value: "{sex}",

                                                onchange: move |event| {
                                                    sex.set(event.value());
                                                },

                                                option { value: "", disabled: true, "Select sex" }

                                                option { value: "male", "Male" }

                                                option { value: "female", "Female" }
                                            }
                                        }

                                        label { class: "flex flex-col gap-1.5",

                                            span { class: "text-sm font-semibold text-gray-700",
                                                "Blood group"
                                            }

                                            select {
                                                class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                                value: "{blood_group}",

                                                onchange: move |event| {
                                                    blood_group.set(event.value());
                                                },

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

                                        span { class: "text-sm font-semibold text-gray-700",
                                            "Date of birth"
                                        }

                                        input {
                                            class: "w-full rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                            r#type: "date",
                                            value: "{date_of_birth}",

                                            oninput: move |event| {
                                                date_of_birth.set(event.value());
                                            },
                                        }
                                    }

                                    div { class: "grid grid-cols-1 gap-4 sm:grid-cols-2",

                                        label { class: "flex flex-col gap-1.5",

                                            span { class: "text-sm font-semibold text-gray-700",
                                                "Height"
                                            }

                                            div { class: "flex items-center gap-2",

                                                input {
                                                    class: "w-full min-w-0 rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                                    r#type: "number",
                                                    step: "1",
                                                    placeholder: "160",
                                                    value: "{height}",

                                                    oninput: move |event| {
                                                        height.set(event.value());
                                                    },
                                                }

                                                span { class: "text-sm text-gray-500",
                                                    "cm"
                                                }
                                            }
                                        }

                                        label { class: "flex flex-col gap-1.5",

                                            span { class: "text-sm font-semibold text-gray-700",
                                                "Weight"
                                            }

                                            div { class: "flex items-center gap-2",

                                                input {
                                                    class: "w-full min-w-0 rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                                    r#type: "number",
                                                    step: "0.1",
                                                    placeholder: "55",
                                                    value: "{weight}",

                                                    oninput: move |event| {
                                                        weight.set(event.value());
                                                    },
                                                }

                                                span { class: "text-sm text-gray-500",
                                                    "kg"
                                                }
                                            }
                                        }
                                    }

                                    label { class: "flex flex-col gap-1.5",

                                        span { class: "text-sm font-semibold text-gray-700",

                                            "Allergies"

                                            span { class: "font-normal text-gray-400",
                                                " (if any)"
                                            }
                                        }

                                        textarea {
                                            class: "min-h-[90px] w-full resize-y rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                            placeholder: "e.g. Penicillin, peanuts...",
                                            value: "{allergies}",

                                            oninput: move |event| {
                                                allergies.set(event.value());
                                            },
                                        }
                                    }

                                    label { class: "flex flex-col gap-1.5",

                                        span { class: "text-sm font-semibold text-gray-700",

                                            "Existing medical conditions"

                                            span { class: "font-normal text-gray-400",
                                                " (if any)"
                                            }
                                        }

                                        textarea {
                                            class: "min-h-[90px] w-full resize-y rounded-md border border-gray-300 bg-white px-3 py-2.5 text-[15px] text-gray-900 outline-none focus:border-blue-600 focus:ring-2 focus:ring-blue-600/10",
                                            placeholder: "e.g. Diabetes, hypertension...",
                                            value: "{medical_conditions}",

                                            oninput: move |event| {
                                                medical_conditions.set(event.value());
                                            },
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

                                        if editing_patient().is_some() {
                                            "Update Patient"
                                        } else {
                                            "Register Patient"
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
