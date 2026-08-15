use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/analysis/")]
    PrescriptionAnalysis {},
    #[route("/details/")]
    MedicineDetails {},
    #[route("/guidance/")]
    SymptomGuidance {},
    #[route("/interaction/")]
    DrugInteraction {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Pill Detect" }
        document::Link { rel: "icon", href: asset!("/assets/logo.svg") }
        document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Router::<Route> {}
    }
}

/// Navbar
#[component]
fn Navbar() -> Element {
    let mut menu_open = use_signal(|| false);

    rsx! {
        div { id: "navbar",
            div { id: "navbar_header",

                Link { to: Route::Home {}, "Pill Detect" }
            }

            // Hamburger button
            button {
                id: "hamburger",
                class: if menu_open() { "active" } else { "" },
                onclick: move |_| menu_open.set(!menu_open()),
                span {}
                span {}
                span {}
            }

            div { id: "navbar_links", class: if menu_open() { "open" } else { "" },
                Link { to: Route::PrescriptionAnalysis {}, "Prescription Analysis" }
                Link { to: Route::MedicineDetails {}, "Medicine Details" }
                Link { to: Route::SymptomGuidance {}, "Symptom Guidance" }
                Link { to: Route::DrugInteraction {}, "Drug Interaction" }
            }
        }
        Outlet::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {}
}

/// Prescription Analysis
#[component]
pub fn PrescriptionAnalysis() -> Element {
    rsx! {
        div { id: "analysis",
            h1 { "Prescription Analysis" }
        }
    }
}

/// Medicine Details
#[component]
pub fn MedicineDetails() -> Element {
    rsx! {
        div { id: "details",
            h1 { "Medicine Details" }
        }
    }
}

/// Symptom Guidance
#[component]
pub fn SymptomGuidance() -> Element {
    rsx! {
        div { id: "guidance",
            h1 { "Symptom Guidance" }
        }
    }
}

/// Drug Interaction
#[component]
pub fn DrugInteraction() -> Element {
    rsx! {
        div { id: "interaction",
            h1 { "Drug Interaction" }
        }
    }
}
