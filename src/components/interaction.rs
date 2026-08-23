use dioxus::prelude::*;

use crate::{
    components::navbar::Navbar,
    modules::{
        database::{
            InteractionHistory, add_interaction_history, clear_interaction_history,
            get_interaction_history, get_patient_id,
        },
        prompts::{DrugInteractionResponse, check_drug_interactions, identify_medicine},
    },
};

#[component]
pub fn DrugInteraction(patient_id: String) -> Element {
    let mut medicine_input = use_signal(String::new);
    let mut selected_medicines = use_signal(Vec::<(String, String)>::new);

    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let mut interaction_loading = use_signal(|| false);
    let mut interaction_results = use_signal(|| Option::<DrugInteractionResponse>::None);

    let mut interaction_history = use_signal(|| Vec::<InteractionHistory>::new());

    // Load interaction history
    use_effect({
        let patient_id = patient_id.clone();

        move || {
            let patient_id = patient_id.clone();

            spawn(async move {
                match get_interaction_history(&patient_id) {
                    Ok(history) => {
                        interaction_history.set(history);
                    }

                    Err(err) => {
                        eprintln!("Failed to load interaction history: {err}");
                    }
                }
            });
        }
    });

    rsx! {
        Navbar { patient_id: patient_id.clone() }

        main {
            class: "min-h-[90vh] max-w-[900px] mx-auto px-6 py-12",

            // Header
            h1 {
                class: "text-[32px] font-bold tracking-tight text-slate-900",
                "Drug Interaction ⚠️"
            }

            p {
                class: "mt-2 mb-8 text-base text-slate-500",
                "Check medicines for interactions and potential safety concerns."
            }

            // Medicine selection
            section {
                class: "rounded-[14px] border border-slate-200 bg-slate-50 p-5",

                h2 {
                    class: "mb-1 text-lg font-semibold text-slate-900",
                    "Select medicines to check"
                }

                p {
                    class: "mb-5 text-sm text-slate-500",
                    "Enter a medicine name or brand name to add it to the safety check."
                }

                form {
                    class: "flex flex-col gap-2.5 sm:flex-row",

                    onsubmit: move |event| {
                        event.prevent_default();

                        let term = medicine_input.read().trim().to_string();

                        if term.is_empty() || loading() {
                            return;
                        }

                        loading.set(true);
                        error.set(None);

                        spawn(async move {
                            match identify_medicine(&term).await {
                                Ok(result) => {
                                    loading.set(false);

                                    if result.found {
                                        let generic =
                                            result.generic.trim().to_string();

                                        let brand =
                                            result.product.trim().to_string();

                                        if generic.is_empty() {
                                            error.set(Some(
                                                "The medicine was identified, but no generic name was returned."
                                                    .to_string(),
                                            ));
                                            return;
                                        }

                                        if !selected_medicines
                                            .read()
                                            .iter()
                                            .any(|(g, _)| {
                                                g.eq_ignore_ascii_case(&generic)
                                            })
                                        {
                                            selected_medicines
                                                .write()
                                                .push((generic, brand));

                                            // Clear previous results because
                                            // the medicine list changed.
                                            interaction_results.set(None);
                                        }

                                        medicine_input.set(String::new());
                                    } else {
                                        error.set(Some(
                                            "Couldn't confidently identify that medicine. Please check again."
                                                .to_string(),
                                        ));
                                    }
                                }

                                Err(err) => {
                                    loading.set(false);
                                    error.set(Some(err.to_string()));
                                }
                            }
                        });
                    },

                    input {
                        class: "min-w-0 flex-1 rounded-[10px] border border-slate-300 bg-white px-3.5 py-3 text-[15px] text-slate-900 outline-none placeholder:text-slate-400 focus:border-blue-600 focus:ring-3 focus:ring-blue-600/10",

                        r#type: "text",
                        placeholder: "e.g. Dolo 650",
                        value: "{medicine_input}",
                        disabled: loading(),

                        oninput: move |event| {
                            medicine_input.set(event.value());
                        },
                    }

                    button {
                        class: "rounded-[10px] bg-blue-600 px-5 py-3 text-sm font-semibold text-white transition hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60",

                        r#type: "submit",
                        disabled: loading(),

                        if loading() {
                            "Identifying..."
                        } else {
                            "Add medicine"
                        }
                    }
                }

                // Error
                if let Some(message) = error() {
                    p {
                        class: "mt-4 rounded-[10px] border border-red-200 bg-red-50 px-4 py-3.5 text-sm text-red-700",
                        "{message}"
                    }
                }

                // Selected medicines
                if !selected_medicines.read().is_empty() {
                    div {
                        class: "mt-6",

                        h3 {
                            class: "mb-3 text-sm font-semibold text-slate-700",
                            "Selected medicines"
                        }

                        div {
                            class: "flex flex-wrap gap-2",

                            for (index, (generic, brand)) in selected_medicines.read().iter().enumerate() {
                                {
                                    let generic = generic.clone();
                                    let brand = brand.clone();

                                    rsx! {
                                        div {
                                            key: "{index}",
                                            class: "flex items-center gap-2 rounded-[10px] border border-slate-200 bg-white px-3.5 py-2.5 text-sm text-slate-700",

                                            span {
                                                class: "font-medium",

                                                if brand.is_empty() {
                                                       "{generic}"
                                                   } else {
                                                       "{generic} ({brand})"
                                                   }
                                            }

                                            button {
                                                class: "flex h-6 w-6 items-center justify-center rounded-full text-slate-400 transition hover:bg-slate-100 hover:text-red-600",

                                                r#type: "button",
                                                aria_label: "Remove {generic}",

                                                onclick: move |_| {
                                                    selected_medicines
                                                        .write()
                                                        .remove(index);

                                                    interaction_results.set(None);
                                                },

                                                "×"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Check interactions
                div {
                    class: "mt-6 border-t border-slate-200 pt-5",

                    button {
                        class: "rounded-[10px] bg-blue-600 px-5 py-3 text-sm font-semibold text-white transition hover:bg-blue-700 disabled:cursor-not-allowed disabled:bg-slate-300",

                        r#type: "button",

                        disabled: selected_medicines.read().is_empty()
                            || interaction_loading(),

                        onclick: move |_| {
                            let medicines: Vec<String> = selected_medicines
                                .read()
                                .iter()
                                .map(|(generic, _)| generic.clone())
                                .collect();

                            let patient_id = patient_id.clone();

                            interaction_loading.set(true);
                            interaction_results.set(None);

                            spawn(async move {
                                let patient =
                                    match get_patient_id(&patient_id) {
                                        Ok(patient) => patient,

                                        Err(err) => {
                                            eprintln!(
                                                "Failed to load patient: {err}"
                                            );
                                            interaction_loading.set(false);
                                            return;
                                        }
                                    };

                                let allergies =
                                    if patient.allergies.trim().is_empty() {
                                        Vec::new()
                                    } else {
                                        vec![patient.allergies]
                                    };

                                let medical_conditions =
                                    if patient
                                        .medical_conditions
                                        .trim()
                                        .is_empty()
                                    {
                                        Vec::new()
                                    } else {
                                        vec![patient.medical_conditions]
                                    };

                                match check_drug_interactions(
                                    medicines.clone(),
                                    allergies,
                                    medical_conditions,
                                )
                                .await
                                {
                                    Ok(result) => {
                                        // Save only the medicines checked.
                                        if let Err(err) =
                                            add_interaction_history(
                                                &patient_id,
                                                medicines,
                                            )
                                        {
                                            eprintln!(
                                                "Failed to save interaction history: {err}"
                                            );
                                        }

                                        // Refresh history.
                                        match get_interaction_history(
                                            &patient_id,
                                        ) {
                                            Ok(history) => {
                                                interaction_history
                                                    .set(history);
                                            }

                                            Err(err) => {
                                                eprintln!(
                                                    "Failed to refresh interaction history: {err}"
                                                );
                                            }
                                        }

                                        interaction_results
                                            .set(Some(result));
                                    }

                                    Err(err) => {
                                        eprintln!(
                                            "Drug interaction check failed: {err}"
                                        );
                                    }
                                }

                                interaction_loading.set(false);
                            });
                        },

                        if interaction_loading() {
                            "Checking..."
                        } else {
                            "Check safety"
                        }
                    }

                    if selected_medicines.read().is_empty() {
                        p {
                            class: "mt-2 text-xs text-slate-400",
                            "Add at least one medicine to check for interactions and safety concerns."
                        }
                    }
                }
            }

            // Results
            if let Some(results) = interaction_results() {
                section {
                    class: "mt-6",

                    div {
                        class: "mb-3",

                        h2 {
                            class: "text-lg font-semibold text-slate-900",
                            "Safety check results"
                        }

                        p {
                            class: "mt-1 text-sm text-slate-500",
                            "Potential interactions and safety concerns identified from the selected medicines and patient information."
                        }
                    }

                    if results.interactions.is_empty() {
                        div {
                            class: "rounded-[14px] border border-green-200 bg-green-50 p-6",

                            div {
                                class: "flex items-start gap-4",

                                div {
                                    class: "flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-green-100 text-lg text-green-700",
                                    "✓"
                                }

                                div {
                                    h3 {
                                        class: "font-semibold text-green-900",
                                        "No interactions found"
                                    }

                                    p {
                                        class: "mt-1 text-sm leading-6 text-green-700",
                                        "No clinically significant interactions or safety concerns were identified for the selected medicine and the patient's recorded information."
                                    }
                                }
                            }
                        }
                    } else {
                        div {
                            class: "space-y-4",

                            for interaction in results.interactions {
                                {
                                    let severity =
                                        interaction.severity.to_lowercase();

                                    let (card_class, badge_class) =
                                        match severity.as_str() {
                                            "high" => (
                                                "border-red-200 bg-red-50",
                                                "bg-red-100 text-red-700",
                                            ),

                                            "moderate" | "medium" => (
                                                "border-yellow-200 bg-yellow-50",
                                                "bg-yellow-100 text-yellow-800",
                                            ),

                                            "low" => (
                                                "border-slate-200 bg-slate-50",
                                                "bg-slate-100 text-slate-700",
                                            ),

                                            _ => (
                                                "border-slate-200 bg-white",
                                                "bg-slate-100 text-slate-700",
                                            ),
                                        };

                                    rsx! {
                                        div {
                                            class: "rounded-[14px] border p-6 {card_class}",

                                            div {
                                                class: "flex flex-wrap items-center justify-between gap-3",

                                                div {
                                                    h3 {
                                                        class: "font-semibold text-slate-900",
                                                        "{interaction.drugs.join(\" ↔ \")}"
                                                    }

                                                    p {
                                                        class: "mt-1 text-xs font-medium uppercase tracking-wide text-slate-500",
                                                        "{interaction.r#type}"
                                                    }
                                                }

                                                span {
                                                    class: "rounded-full px-3 py-1 text-xs font-semibold {badge_class}",
                                                    "{interaction.severity}"
                                                }
                                            }

                                            div {
                                                class: "mt-5 space-y-4",

                                                div {
                                                    h4 {
                                                        class: "text-sm font-semibold text-slate-700",
                                                        "Interaction"
                                                    }

                                                    p {
                                                        class: "mt-1 text-sm leading-6 text-slate-600",
                                                        "{interaction.interaction}"
                                                    }
                                                }

                                                div {
                                                    h4 {
                                                        class: "text-sm font-semibold text-slate-700",
                                                        "Possible effects"
                                                    }

                                                    p {
                                                        class: "mt-1 text-sm leading-6 text-slate-600",
                                                        "{interaction.effects}"
                                                    }
                                                }

                                                div {
                                                    h4 {
                                                        class: "text-sm font-semibold text-slate-700",
                                                        "Recommendation"
                                                    }

                                                    p {
                                                        class: "mt-1 text-sm leading-6 text-slate-600",
                                                        "{interaction.recommendation}"
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

            // Past interaction checks
            if !interaction_history.read().is_empty() {
                section {
                    class: "mt-8",

                    div {
                        class: "mb-4 flex items-center justify-between gap-4",

                        div {
                            h2 {
                                class: "text-lg font-semibold text-slate-900",
                                "Past interaction checks"
                            }

                            p {
                                class: "mt-1 text-sm text-slate-500",
                                "Previously checked medicines for this patient."
                            }
                        }

                        button {
                            class: "shrink-0 rounded-[9px] border border-slate-200 px-3 py-2 text-sm font-medium text-slate-600 transition hover:border-red-200 hover:bg-red-50 hover:text-red-600",

                            r#type: "button",

                            onclick: {
                                let patient_id = patient_id.clone();

                                move |_| {
                                    match clear_interaction_history(&patient_id) {
                                        Ok(_) => {
                                            interaction_history.set(Vec::new());
                                        }

                                        Err(err) => {
                                            eprintln!(
                                                "Failed to clear interaction history: {err}"
                                            );
                                        }
                                    }
                                }
                            },

                            "Clear"
                        }
                    }

                    div {
                        class: "space-y-3",

                        for (index, history) in interaction_history.read().iter().enumerate() {
                            {
                                let medicines = history.medicines.clone();
                                let display_medicines =
                                    medicines.join(" • ");

                                rsx! {
                                    div {
                                        key: "{index}",
                                        class: "flex flex-col gap-3 rounded-[14px] border border-slate-200 bg-white p-4 sm:flex-row sm:items-center sm:justify-between",

                                        div {
                                            p {
                                                class: "text-sm font-medium text-slate-800",
                                                "{display_medicines}"
                                            }

                                            p {
                                                class: "mt-1 text-xs text-slate-400",
                                                "{medicines.len()} medicine(s) checked"
                                            }
                                        }

                                        button {
                                            class: "self-start rounded-[9px] border border-slate-200 px-4 py-2 text-sm font-medium text-slate-700 transition hover:bg-blue-700 hover:text-white sm:self-auto",

                                            r#type: "button",

                                            onclick: move |_| {
                                                let medicines =
                                                    medicines.clone();

                                                selected_medicines.set(
                                                    medicines
                                                        .into_iter()
                                                        .map(|medicine| {
                                                            (
                                                                medicine.clone(),
                                                                String::new(),
                                                            )
                                                        })
                                                        .collect(),
                                                );

                                                interaction_results
                                                    .set(None);
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
