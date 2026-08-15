use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer",

            p {
                "Disclaimer: Pill Detect is not intended to provide medical diagnosis, treatment, or professional medical advice."
            }
        }
    }
}
