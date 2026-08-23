mod components;
mod modules;

use dioxus::prelude::*;

use components::{
    dashboard::Dashboard, footer::Footer, guidance::Guidance, homepage::Homepage,
    information::Information, interaction::DrugInteraction, prescription::PrescriptionAnalysis,
};

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    Homepage {},

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
fn App() -> Element {
    use_effect(|| {
        spawn(async {
            crate::modules::api::get_api_key().await.ok();
        });
    });

    rsx! {
        document::Title { "Pill Detect" }
        document::Link {
            rel: "icon",
            href: "data:image/svg+xml,
          <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'>
            <text y='0.9em' font-size='90'>🩺</text>
          </svg>",
        }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        Router::<Route> {}
        Footer {}
    }
}
