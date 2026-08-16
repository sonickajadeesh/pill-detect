use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer",

            p {
                "Disclaimer: This is not intended to provide professional medical consultation."
            }
        }
    }
}
