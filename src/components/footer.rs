use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "border-t border-gray-200 px-4 py-2 text-center text-xs leading-relaxed text-gray-500",

            p { "Disclaimer: This is not intended to provide professional medical consultation." }
        }
    }
}
