use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar(patient_id: String) -> Element {
    let mut show_menu = use_signal(|| false);

    rsx! {
        nav { class: "relative flex h-14 items-center border-b border-slate-200 bg-white px-4",

            // Back to dashboard
            Link {
                to: Route::Dashboard {
                    patient_id: patient_id.clone(),
                },
                class: "flex items-center gap-1 text-sm font-medium text-slate-600 transition-colors hover:text-slate-900",

                "← "
                " Back"
            }

            // Centered header
            div { class: "absolute left-1/2 -translate-x-1/2",

                Link {
                    to: Route::Homepage {},
                    class: "text-lg font-semibold text-slate-900 hover:text-slate-700",

                    "🩺 Pill Detect"
                }
            }

            // Navigation
            div { class: "relative ml-auto",

                button {
                    class: "flex h-9 w-9 items-center justify-center rounded-md text-xl text-slate-600 transition-colors hover:bg-slate-100 hover:text-slate-900",
                    r#type: "button",

                    onclick: move |_| show_menu.toggle(),

                    "☰"
                }

                if show_menu() {
                    div { class: "absolute right-0 top-11 z-50 w-50 rounded-lg border border-slate-200 bg-white p-1 shadow-lg",

                        Link {
                            to: Route::Information {
                                patient_id: patient_id.clone(),
                            },
                            class: "block rounded-md px-4 py-2.5 text-sm font-medium text-slate-600 hover:bg-slate-100 hover:text-slate-900",

                            "Medicine Details 🔎"
                        }

                        Link {
                            to: Route::Guidance {
                                patient_id: patient_id.clone(),
                            },
                            class: "block rounded-md px-4 py-2.5 text-sm font-medium text-slate-600 hover:bg-slate-100 hover:text-slate-900",

                            "Symptom Guidance 💬"
                        }
                    }
                }
            }
        }
    }
}
