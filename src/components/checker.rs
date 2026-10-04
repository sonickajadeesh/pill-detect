use crate::modules::bodymap::{BodyMap, BodyParts};
use dioxus::prelude::*;

#[component]
pub fn SymptomChecker() -> Element {
    let mut selected = use_signal(|| None::<BodyParts>);
    let mut selected_symptoms = use_signal(Vec::<String>::new);

    rsx! {
        div { class: "relative flex min-h-[96vh] flex-col items-center justify-center px-5 py-10 text-center",

            BodyMap {
                selected: selected(),
                on_select: move |part| {
                    selected.set(Some(part));
                    selected_symptoms.set(Vec::new());
                },
            }

            if let Some(part) = selected() {
                div {
                    h2 { "{part.label()}" }

                    p { "What symptoms are you experiencing?" }

                    for symptom in part.symptoms() {
                        label {
                            input {
                                r#type: "checkbox",
                                value: "{symptom}",
                                onchange: move |_| {
                                    let symptom = symptom.to_string();

                                    selected_symptoms
                                        .with_mut(|symptoms| {
                                            if symptoms.contains(&symptom) {
                                                symptoms.retain(|s| s != &symptom);
                                            }
                                                symptoms.push(symptom);
                                            }
                                        });
                                },
                            }

                            "{symptom}"
                        }
                    }
                }
            }
        }
    }
}
