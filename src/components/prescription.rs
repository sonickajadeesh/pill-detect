use chrono::Local;
use dioxus::prelude::*;

use crate::{
    components::navbar::Navbar,
    modules::{
        database::{
            Prescription, add_prescription, delete_prescription, get_prescriptions,
            update_prescription,
        },
        utilities::format_date,
    },
};

#[component]
pub fn PrescriptionAnalysis(patient_id: String) -> Element {
    let mut prescriptions = use_signal(Vec::<Prescription>::new);
    let mut show_form = use_signal(|| false);
    let mut editing_id = use_signal(|| None::<String>);

    let mut reason = use_signal(String::new);
    let mut prescription_text = use_signal(String::new);
    let mut expiry_date = use_signal(String::new);
    let mut error_message = use_signal(|| None::<String>);

    let patient_id_for_load = patient_id.clone();

    use_effect(move || match get_prescriptions(&patient_id_for_load) {
        Ok(data) => {
            prescriptions.set(data);
            error_message.set(None);
        }
        Err(err) => {
            error_message.set(Some(err));
        }
    });

    let mut reset_form = move || {
        reason.set(String::new());
        prescription_text.set(String::new());
        expiry_date.set(String::new());
        editing_id.set(None);
        error_message.set(None);
    };

    let mut open_add_form = move || {
        reset_form();
        show_form.set(true);
    };

    let mut open_edit_form = move |prescription: Prescription| {
        editing_id.set(Some(prescription.id));
        reason.set(prescription.reason);
        prescription_text.set(prescription.prescription_text);
        expiry_date.set(prescription.expiry_date);
        error_message.set(None);
        show_form.set(true);
    };

    let mut close_form = move || {
        show_form.set(false);
        reset_form();
    };

    let save_prescription = {
        let patient_id = patient_id.clone();

        move |_| {
            let reason_value = reason().trim().to_string();
            let prescription_text_value = prescription_text().trim().to_string();
            let expiry_date_value = expiry_date().trim().to_string();

            if reason_value.is_empty() {
                error_message.set(Some("Please enter a reason.".to_string()));
                return;
            }

            if prescription_text_value.is_empty() {
                error_message.set(Some("Please enter the prescription.".to_string()));
                return;
            }

            if expiry_date_value.is_empty() {
                error_message.set(Some("Please select an expiry date.".to_string()));
                return;
            }

            let result = match editing_id() {
                Some(id) => {
                    let created_at = prescriptions()
                        .iter()
                        .find(|prescription| prescription.id == id)
                        .map(|prescription| prescription.created_at.clone())
                        .unwrap_or_default();

                    update_prescription(
                        &patient_id,
                        Prescription {
                            id,
                            reason: reason_value,
                            prescription_text: prescription_text_value,
                            created_at,
                            expiry_date: expiry_date_value,
                        },
                    )
                }

                None => {
                    let id = format!("{}-{}", js_sys::Date::now() as u64, prescriptions().len());

                    let created_at = js_sys::Date::new_0()
                        .to_iso_string()
                        .as_string()
                        .unwrap_or_default();

                    add_prescription(
                        &patient_id,
                        Prescription {
                            id,
                            reason: reason_value,
                            prescription_text: prescription_text_value,
                            created_at,
                            expiry_date: expiry_date_value,
                        },
                    )
                }
            };

            match result {
                Ok(_) => {
                    match get_prescriptions(&patient_id) {
                        Ok(data) => prescriptions.set(data),
                        Err(err) => {
                            error_message.set(Some(err));
                            return;
                        }
                    }

                    show_form.set(false);
                    reset_form();
                }

                Err(err) => {
                    error_message.set(Some(err));
                }
            }
        }
    };

    let today = Local::now().format("%Y-%m-%d").to_string();

    let active_prescriptions: Vec<Prescription> = prescriptions()
        .iter()
        .filter(|prescription| prescription.expiry_date >= today)
        .cloned()
        .collect();

    let expired_prescriptions: Vec<Prescription> = prescriptions()
        .iter()
        .filter(|prescription| prescription.expiry_date < today)
        .cloned()
        .collect();

    rsx! {
        Navbar { patient_id: patient_id.clone() }

        main { class: "min-h-[90vh] max-w-[900px] mx-auto px-5 py-8",

            div { class: "mb-6 flex items-center justify-between",

                div {
                    h1 { class: "text-2xl font-semibold text-gray-900", "Prescriptions" }

                    p { class: "mt-1 text-sm text-gray-500", "View and manage patient prescriptions." }
                }

                button {
                    class: "rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700",
                    onclick: move |_| open_add_form(),
                    "+ Add Prescription"
                }
            }

            if let Some(error) = error_message() {
                div { class: "mb-5 rounded-lg border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700",
                    "{error}"
                }
            }

            if prescriptions().is_empty() {
                div { class: "rounded-xl border border-dashed border-gray-300 px-6 py-12 text-center",

                    p { class: "text-sm text-gray-500", "No prescriptions have been added." }

                    button {
                        class: "mt-3 text-sm font-medium text-gray-900 underline",
                        onclick: move |_| open_add_form(),
                        "Add the first prescription"
                    }
                }
            } else {
                div { class: "space-y-8",

                    // Active prescriptions
                    if !active_prescriptions.is_empty() {
                        section {
                            h2 { class: "mb-4 text-lg font-semibold text-gray-900",
                                "Active Prescriptions"
                            }

                            div { class: "space-y-4",

                                for prescription in active_prescriptions {
                                    div {
                                        key: "{prescription.id}",
                                        class: "rounded-xl border border-gray-200 bg-white p-5 shadow-sm",

                                        div { class: "flex items-start justify-between gap-4",

                                            div {
                                                h2 { class: "font-semibold text-gray-900",
                                                    "{prescription.reason}"
                                                }

                                                p { class: "mt-1 text-xs text-gray-500",
                                                    "Created: {format_date(&prescription.created_at)}"
                                                }
                                            }

                                            div { class: "flex gap-2",

                                                button {
                                                    class: "rounded-md px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-100",

                                                    onclick: {
                                                        let prescription = prescription.clone();

                                                        move |_| {
                                                            open_edit_form(prescription.clone());
                                                        }
                                                    },

                                                    "Edit"
                                                }

                                                button {
                                                    class: "rounded-md px-3 py-1.5 text-sm text-red-600 hover:bg-red-50",

                                                    onclick: {
                                                        let patient_id = patient_id.clone();
                                                        let prescription_id = prescription.id.clone();

                                                        move |_| {
                                                            match delete_prescription(&patient_id, &prescription_id) {
                                                                Ok(_) => {
                                                                    match get_prescriptions(&patient_id) {
                                                                        Ok(data) => prescriptions.set(data),
                                                                        Err(err) => {
                                                                            error_message.set(Some(err));
                                                                        }
                                                                    }
                                                                }

                                                                Err(err) => {
                                                                    error_message.set(Some(err));
                                                                }
                                                            }
                                                        }
                                                    },

                                                    "Delete"
                                                }
                                            }
                                        }

                                        div { class: "mt-4 rounded-lg bg-gray-100 p-4",

                                            p { class: "whitespace-pre-wrap text-sm leading-6 text-gray-800",
                                                "{prescription.prescription_text}"
                                            }
                                        }

                                        div { class: "mt-4 flex items-center gap-1 text-sm",

                                            span { class: "text-gray-500", "Expires:" }

                                            span { class: "text-gray-900",
                                                "{format_date(&prescription.expiry_date)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Expired prescriptions
                    if !expired_prescriptions.is_empty() {
                        section {
                            h2 { class: "mb-4 text-lg font-semibold text-gray-9 00",
                                "Expired Prescriptions"
                            }

                            div { class: "space-y-4",

                                for prescription in expired_prescriptions {
                                    div {
                                        key: "{prescription.id}",
                                        class: "rounded-xl border border-gray-200 bg-gray-50 p-5",

                                        div { class: "flex items-start justify-between gap-4",

                                            div {
                                                h2 { class: "font-semibold text-gray-600",
                                                    "{prescription.reason}"
                                                }

                                                p { class: "mt-1 text-xs text-gray-400",
                                                    "Created: {format_date(&prescription.created_at)}"
                                                }
                                            }

                                            button {
                                                class: "rounded-md px-3 py-1.5 text-sm text-red-600 hover:bg-red-100",

                                                onclick: {
                                                    let patient_id = patient_id.clone();
                                                    let prescription_id = prescription.id.clone();

                                                    move |_| {
                                                        match delete_prescription(&patient_id, &prescription_id) {
                                                            Ok(_) => {
                                                                match get_prescriptions(&patient_id) {
                                                                    Ok(data) => prescriptions.set(data),
                                                                    Err(err) => {
                                                                        error_message.set(Some(err));
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                error_message.set(Some(err));
                                                            }
                                                        }
                                                    }
                                                },

                                                "Delete"
                                            }
                                        }

                                        div { class: "mt-4 rounded-lg bg-white p-4",

                                            p { class: "whitespace-pre-wrap text-sm leading-6 text-gray-600",
                                                "{prescription.prescription_text}"
                                            }
                                        }

                                        div { class: "mt-4 flex items-center gap-1 text-sm",

                                            span { class: "text-gray-400", "Expired:" }

                                            span { class: "text-gray-600",
                                                "{format_date(&prescription.expiry_date)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if show_form() {
                div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/40 px-4",

                    div { class: "w-full max-w-lg rounded-2xl bg-white p-6 shadow-xl",

                        div { class: "mb-6 flex items-center justify-between",

                            h2 { class: "text-lg font-semibold text-gray-900",

                                if editing_id().is_some() {
                                    "Edit Prescription"
                                } else {
                                    "Add Prescription"
                                }
                            }

                            button {
                                class: "text-gray-400 hover:text-gray-700",
                                onclick: move |_| close_form(),
                                "✕"
                            }
                        }

                        div { class: "space-y-5",

                            div {
                                label { class: "mb-2 block text-sm font-medium text-gray-700",
                                    "Reason"
                                }

                                input {
                                    class: "w-full rounded-lg border border-gray-300 px-3 py-2.5 text-sm outline-none focus:border-black",
                                    r#type: "text",
                                    placeholder: "e.g. Respiratory infection",
                                    value: "{reason}",
                                    oninput: move |event| {
                                        reason.set(event.value());
                                    },
                                }
                            }

                            div {
                                label { class: "mb-2 block text-sm font-medium text-gray-700",
                                    "Prescription"
                                }

                                textarea {
                                    class: "min-h-32 w-full resize-y rounded-lg border border-gray-300 px-3 py-2.5 text-sm outline-none focus:border-black",
                                    placeholder: "e.g. Amoxicillin 3x day for 5 days",
                                    value: "{prescription_text}",
                                    oninput: move |event| {
                                        prescription_text.set(event.value());
                                    },
                                }
                            }

                            div {
                                label { class: "mb-2 block text-sm font-medium text-gray-700",
                                    "Expiry date"
                                }

                                input {
                                    class: "w-full rounded-lg border border-gray-300 px-3 py-2.5 text-sm outline-none focus:border-black",
                                    r#type: "date",
                                    value: "{expiry_date}",
                                    oninput: move |event| {
                                        expiry_date.set(event.value());
                                    },
                                }
                            }
                        }

                        div { class: "mt-6 flex justify-end gap-3",

                            button {
                                class: "rounded-lg border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50",
                                onclick: move |_| close_form(),
                                "Cancel"
                            }

                            button {
                                class: "rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700",
                                onclick: save_prescription,

                                if editing_id().is_some() {
                                    "Save Changes"
                                } else {
                                    "Add Prescription"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
