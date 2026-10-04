mod components;
mod modules;

use components::{
    auth::AuthPage, checker::SymptomChecker, dashboard::Dashboard, footer::Footer,
    guidance::Guidance, homepage::Homepage, information::Information, interaction::DrugInteraction,
    prescription::PrescriptionAnalysis,
};
use dioxus::prelude::*;
use modules::firebase_auth::AuthService;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},

    #[route("/test")]
    SymptomChecker {},

    #[route("/:patient_id/")]
    Dashboard { patient_id: String },

    #[route("/:patient_id/guidance")]
    Guidance { patient_id: String },

    #[route("/:patient_id/information/")]
    Information { patient_id: String },

    #[route("/:patient_id/interaction/")]
    DrugInteraction { patient_id: String },

    #[route("/:patient_id/prescriptions/")]
    PrescriptionAnalysis { patient_id: String },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn Home() -> Element {
    let mut user = use_signal(|| AuthService::new().current_user().ok().flatten());

    if user.read().is_some() {
        rsx! {
            Homepage {}
        }
    } else {
        rsx! {
            AuthPage {
                on_authenticated: move |authenticated_user| {
                    user.set(Some(authenticated_user));
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Title { "Pill Detect" }

        document::Link {
            rel: "icon",
            href: "data:image/svg+xml,
            <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
                <text y='0.9em' font-size='90'>🩺</text>
            </svg>",
        }

        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/tailwind.css")
        }

        Router::<Route> {}

        Footer {}
    }
}
