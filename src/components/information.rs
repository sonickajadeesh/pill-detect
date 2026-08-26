use dioxus::prelude::*;

use crate::{
    components::navbar::Navbar,
    modules::{
        database::{SearchHistory, add_search_history, clear_search_history, get_search_history},
        prompts::{
            MedicineInformation, identify_medicine, identify_medicine_image, research_medicine,
        },
        utilities::read_file_bytes,
    },
};

#[component]
pub fn Information(patient_id: String) -> Element {
    let mut search_term = use_signal(String::new);

    // Separate loading states for the different operations.
    let mut searching = use_signal(|| false);
    let mut researching = use_signal(|| false);

    let mut error = use_signal(|| Option::<String>::None);

    let mut medicine = use_signal(|| Option::<(String, String)>::None);
    let mut information = use_signal(|| Option::<MedicineInformation>::None);

    let mut history = use_signal(Vec::<SearchHistory>::new);

    // Tracks whether the current medicine was selected from search history.
    let mut from_history = use_signal(|| false);

    // Clone the patient ID for the history-loading effect.
    let history_patient_id = patient_id.clone();

    // Load search history for the selected patient.
    use_effect(move || match get_search_history(&history_patient_id) {
        Ok(saved_history) => {
            history.set(saved_history);
        }

        Err(err) => {
            eprintln!("Failed to load search history: {err}");
        }
    });

    // Clone the patient ID for the research button.
    let research_patient_id = patient_id.clone();

    // Clone the patient ID for the clear button.
    let clear_patient_id = patient_id.clone();

    // Clone the patient ID for image identification.
    let image_patient_id = patient_id.clone();

    rsx! {
        Navbar { patient_id: patient_id.clone() }

        main { class: "mx-auto min-h-[90vh] max-w-[900px] px-6 py-12",

            h1 { class: "text-[32px] font-bold tracking-tight text-slate-900",
                "Medicine Information 🔎"
            }

            p { class: "mt-2 mb-8 text-base text-slate-500",
                "Search for a medicine or identify one from a photo."
            }

            // Search section
            section { class: "rounded-[14px] border border-slate-200 bg-slate-50 p-5",

                form {
                    class: "flex flex-col gap-2.5",

                    onsubmit: move |event| {
                        event.prevent_default();

                        let term = search_term().trim().to_string();

                        if term.is_empty() {
                            return;
                        }

                        searching.set(true);
                        error.set(None);
                        medicine.set(None);
                        information.set(None);
                        from_history.set(false);

                        spawn(async move {
                            match identify_medicine(&term).await {
                                Ok(result) => {
                                    searching.set(false);

                                    if result.found {
                                        medicine.set(Some((result.product, result.generic)));
                                    } else {
                                        error
                                            .set(
                                                Some(
                                                    "Couldn't confidently identify that medicine. Please check again."
                                                        .to_string(),
                                                ),
                                            );
                                    }
                                }
                                Err(err) => {
                                    searching.set(false);
                                    error.set(Some(err.to_string()));
                                }
                            }
                        });
                    },

                    // Search controls
                    div { class: "flex min-w-0 flex-col gap-2.5 sm:flex-row",

                        // Medicine search input
                        input {
                            class: "min-w-0 flex-1 rounded-[10px] border border-slate-300 bg-white px-3.5 py-3 text-[15px] text-slate-900 outline-none placeholder:text-slate-400 focus:border-blue-600 focus:ring-3 focus:ring-blue-600/10",

                            r#type: "text",
                            placeholder: "Type a medicine name",
                            value: "{search_term}",

                            oninput: move |event| {
                                search_term.set(event.value());
                                from_history.set(false);
                            },
                        }

                        // Search + Upload buttons
                        div { class: "flex w-full gap-2.5 sm:w-auto",

                            // Search button
                            button {
                                class: "flex-1 rounded-[10px] bg-blue-600 px-5 py-3 text-sm font-semibold text-white transition hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60 sm:flex-none",

                                r#type: "submit",
                                disabled: searching() || researching(),

                                if searching() {
                                    div { class: "mx-auto h-4 w-4 animate-spin rounded-full border-2 border-white/40 border-t-white" }
                                } else {
                                    "Search 🔍︎"
                                }
                            }

                            // Upload button
                            button {
                                class: "flex-1 rounded-[10px] border border-slate-300 bg-white px-5 py-3 text-sm font-semibold text-slate-700 transition hover:border-blue-300 hover:bg-blue-50 hover:text-blue-700 disabled:cursor-not-allowed disabled:opacity-60 sm:flex-none",

                                r#type: "button",
                                disabled: searching() || researching(),

                                onclick: move |_| {
                                    if !searching() && !researching() {
                                        document::eval(
                                            r#"document.getElementById("medicine-image-upload").click();"#,
                                        );
                                    }
                                },

                                if searching() {
                                    "Identifying..."
                                } else if researching() {
                                    "Loading..."
                                } else {
                                    "Upload ↑"
                                }
                            }
                        }
                    }

                    // Hidden image input
                    input {
                        id: "medicine-image-upload",
                        class: "hidden",

                        r#type: "file",
                        accept: "image/*",

                        onchange: move |event| {
                            if let Some(file) = event.files().first() {
                                let file = file.clone();

                                let mime_type = match file.name().rsplit('.').next() {
                                    Some("png") | Some("PNG") => "image/png",
                                    Some("webp") | Some("WEBP") => "image/webp",
                                    Some("jpg") | Some("JPG")
                                    | Some("jpeg") | Some("JPEG") => "image/jpeg",
                                    _ => "image/jpeg",
                                };

                                searching.set(true);
                                researching.set(false);
                                error.set(None);
                                medicine.set(None);
                                information.set(None);
                                from_history.set(false);

                                let patient_id = image_patient_id.clone();

                                spawn(async move {
                                    let result = match read_file_bytes(&file).await {
                                        Ok(bytes) => identify_medicine_image(&bytes, mime_type).await,
                                        Err(err) => Err(err.into()),
                                    };
                                    match result {
                                        Ok(result) => {
                                            searching.set(false);
                                            if result.found && !result.product.trim().is_empty()
                                                && !result.generic.trim().is_empty()
                                            {
                                                let product_name = result.product;
                                                let generic_name = result.generic;

                                                search_term.set(product_name.clone());

                                                medicine
                                                    .set(Some((product_name.clone(), generic_name.clone())));
                                                researching.set(true);
                                                match research_medicine(&generic_name).await {
                                                    Ok(info) => {
                                                        information.set(Some(info));
                                                        researching.set(false);

                                                        let search = SearchHistory {
                                                            product: product_name,
                                                            generic: generic_name,
                                                        };
                                                        match add_search_history(&patient_id, search) {
                                                            Ok(()) => {
                                                                match get_search_history(&patient_id) {
                                                                    Ok(updated_history) => {
                                                                        history.set(updated_history);
                                                                    }

                                                                    Err(err) => {
                                                                        eprintln!("Failed to reload search history: {err}");
                                                                    }
                                                                }
                                                            }

                                                            Err(err) => {
                                                                eprintln!("Failed to save search history: {err}");
                                                            }
                                                        }
                                                    }

                                                    Err(err) => {
                                                        researching.set(false);
                                                        error.set(Some(err.to_string()));
                                                    }
                                                }
                                            } else {
                                                error
                                                    .set(
                                                        Some(
                                                            "Couldn't confidently identify the medicine from this image. Please upload a clearer photo showing the medicine label."
                                                                .to_string(),
                                                        ),
                                                    );
                                            }
                                        }
                                        Err(err) => {
                                            searching.set(false);
                                            researching.set(false);
                                            error.set(Some(format!("Failed to identify medicine: {err}")));
                                        }
                                    }
                                });
                            }
                        },
                    }
                }
            }

            // Error message
            if let Some(message) = error() {
                p { class: "mt-4 rounded-[10px] border border-red-200 bg-red-50 px-4 py-3.5 text-sm text-red-700",
                    "{message}"
                }
            }

            // Search result
            if let Some((product, generic)) = medicine() {
                section { class: "mt-6 rounded-[14px] border border-slate-200 bg-white p-5 sm:p-7",

                    // Research result
                    if let Some(info) = information() {

                        // Medicine heading
                        div { class: "flex items-start justify-between gap-3 border-b border-slate-100 pb-4",

                            div {
                                h2 { class: "mt-1 text-[22px] font-semibold text-slate-900",
                                    "{generic}"
                                }

                                p { class: "mt-1 text-sm text-slate-500", "{product}" }
                            }

                            if info.prescription {
                                span { class: "mt-2 shrink-0 rounded-full bg-amber-100 px-2.5 py-1 text-xs font-semibold text-amber-700",
                                    "Prescription Required"
                                }
                            } else {
                                span { class: "mt-2 shrink-0 rounded-full bg-emerald-100 px-2.5 py-1 text-xs font-semibold text-emerald-700",
                                    "No Prescription"
                                }
                            }
                        }

                        div { class: "mt-5 space-y-3",

                            // Uses
                            div { class: "rounded-[12px] border border-slate-200 bg-slate-50 p-4",

                                p { class: "text-xs font-semibold uppercase tracking-wide text-slate-400",
                                    "Uses"
                                }

                                p { class: "mt-1.5 text-sm leading-6 text-slate-700",
                                    "{info.uses}"
                                }
                            }

                            // Dosage
                            div { class: "rounded-[12px] border border-slate-200 bg-slate-50 p-4",

                                p { class: "text-xs font-semibold uppercase tracking-wide text-slate-400",
                                    "Dosage"
                                }

                                p { class: "mt-1.5 text-sm leading-6 text-slate-700",
                                    "{info.dosage}"
                                }
                            }

                            // Side effects
                            div { class: "rounded-[12px] border border-slate-200 bg-slate-50 p-4",

                                p { class: "text-xs font-semibold uppercase tracking-wide text-slate-400",
                                    "Side Effects"
                                }

                                p { class: "mt-1.5 text-sm leading-6 text-slate-700",
                                    "{info.side_effects}"
                                }
                            }

                            // Warnings
                            div { class: "rounded-[12px] border border-slate-200 bg-slate-50 p-4",

                                p { class: "text-xs font-semibold uppercase tracking-wide text-slate-400",
                                    "Warnings"
                                }

                                p { class: "mt-1.5 text-sm leading-6 text-slate-700",
                                    "{info.warnings}"
                                }
                            }
                        }
                    } else if from_history() {
                        p { class: "py-6 text-sm text-slate-500", "Loading..." }
                    } else if researching() {
                        div { class: "flex items-center gap-2 py-6 text-sm text-slate-500",

                            div { class: "h-4 w-4 animate-spin rounded-full border-2 border-slate-300 border-t-blue-600" }

                            "Loading medicine information..."
                        }
                    } else {
                        h2 { class: "mt-0 text-md text-slate-900",
                            "Is this the medicine you're looking for?"
                        }

                        div { class: "mt-4 rounded-[10px] border border-slate-200 bg-slate-50 px-4 py-3",

                            div { class: "font-medium text-slate-900", "{generic}" }

                            div { class: "mt-0.5 text-sm text-slate-500", "{product}" }
                        }

                        button {
                            class: "mt-6 rounded-[10px] bg-blue-600 px-5 py-2.5 text-sm font-semibold text-white transition hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60",

                            r#type: "button",
                            disabled: researching(),

                            onclick: {
                                let patient_id = research_patient_id.clone();

                                move |_| {
                                    let product_name = product.clone();
                                    let generic_name = generic.clone();
                                    let patient_id = patient_id.clone();

                                    researching.set(true);
                                    error.set(None);

                                    spawn(async move {
                                        match research_medicine(&generic_name).await {
                                            Ok(result) => {
                                                information.set(Some(result));
                                                researching.set(false);

                                                let search = SearchHistory {
                                                    product: product_name,
                                                    generic: generic_name,
                                                };
                                                match add_search_history(&patient_id, search) {
                                                    Ok(()) => {
                                                        match get_search_history(&patient_id) {
                                                            Ok(updated_history) => {
                                                                history.set(updated_history);
                                                            }

                                                            Err(err) => {
                                                                eprintln!("Failed to reload search history: {err}");
                                                            }
                                                        }
                                                    }

                                                    Err(err) => {
                                                        eprintln!("Failed to save search history: {err}");
                                                    }
                                                }
                                            }

                                            Err(err) => {
                                                researching.set(false);
                                                error.set(Some(err.to_string()));
                                            }
                                        }
                                    });
                                }
                            },

                            if researching() {
                                div { class: "flex items-center justify-center gap-2",

                                    div { class: "h-4 w-4 animate-spin rounded-full border-2 border-white/40 border-t-white" }

                                    "Searching..."
                                }
                            } else {
                                "Yes, continue"
                            }
                        }
                    }
                }
            }

            // Past search history
            if !history().is_empty() {
                section { class: "mt-6",

                    div { class: "mb-3 flex items-center justify-between",

                        h2 { class: "text-lg font-semibold text-slate-900", "Past Searches" }

                        button {
                            class: "rounded-[9px] border border-slate-200 px-3 py-1.5 text-sm text-slate-500 transition hover:border-red-200 hover:bg-red-50 hover:text-red-600",

                            r#type: "button",

                            onclick: {
                                let patient_id = clear_patient_id.clone();

                                move |_| {
                                    match clear_search_history(&patient_id) {
                                        Ok(()) => {
                                            history.set(Vec::new());
                                        }

                                        Err(err) => {
                                            eprintln!("Failed to clear search history: {err}");
                                        }
                                    }
                                }
                            },

                            "Clear"
                        }
                    }

                    div { class: "space-y-2",

                        for (_, history) in history().iter().enumerate() {
                            {
                                let product = history.product.clone();
                                let generic = history.generic.clone();

                                rsx! {
                                    div { class: "flex flex-col gap-3 rounded-[14px] border border-slate-200 bg-white p-4 sm:flex-row sm:items-center sm:justify-between",

                                        div {
                                            p { class: "text-sm font-medium text-slate-800", "{generic}" }

                                            p { class: "mt-1 text-xs text-slate-400", "{product}" }
                                        }

                                        button {
                                            class: "self-start rounded-[9px] border border-slate-200 px-4 py-2 text-sm font-medium text-slate-700 transition hover:bg-blue-700 hover:text-white sm:self-auto",

                                            r#type: "button",
                                            disabled: researching(),

                                            onclick: {
                                                let product = product.clone();
                                                let generic = generic.clone();

                                                move |_| {
                                                    let product_name = product.clone();
                                                    let generic_name = generic.clone();

                                                    search_term.set(product_name.clone());
                                                    medicine.set(Some((product_name, generic_name.clone())));
                                                    error.set(None);
                                                    from_history.set(true);
                                                    researching.set(true);

                                                    spawn(async move {
                                                        match research_medicine(&generic_name).await {
                                                            Ok(result) => {
                                                                information.set(Some(result));
                                                                researching.set(false);
                                                            }

                                                            Err(err) => {
                                                                researching.set(false);
                                                                error.set(Some(err.to_string()));
                                                            }
                                                        }
                                                    });
                                                }
                                            },

                                            "->"
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
