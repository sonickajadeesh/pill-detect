use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar(patient_id: String) -> Element {
    rsx! {
        nav {
            class: "flex h-14 items-center justify-between border-b border-slate-200 bg-white px-4",

            div {
                class: "flex items-center gap-2",

                Link {
                    to: Route::Homepage {},
                    class: "text-lg font-semibold text-slate-900 hover:text-slate-700",
                    "🩺 Pill Detect"
                }
            }

            div {
                class: "flex items-center gap-1 rounded-lg bg-slate-100 p-1",

                Link {
                    to: Route::Information {
                        patient_id: patient_id.clone(),
                    },
                    class: "rounded-md px-4 py-1.5 text-sm font-medium text-slate-600 transition-colors hover:bg-white hover:text-slate-900 hover:shadow-sm",

                    "Search 🔎"
                }

                Link {
                    to: Route::Guidance {
                        patient_id: patient_id.clone(),
                    },
                    class: "rounded-md px-4 py-1.5 text-sm font-medium text-slate-600 transition-colors hover:bg-white hover:text-slate-900 hover:shadow-sm",

                    "Guidance 💬"
                }
            }
        }
    }
}
