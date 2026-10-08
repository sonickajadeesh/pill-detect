use crate::{
    components::navbar::Navbar,
    modules::{geolocation::reverse_geocode, prompts::find_doctors},
};
use dioxus::prelude::*;
use dioxus_sdk_geolocation::{PowerMode, init_geolocator, use_geolocation};

#[component]
pub fn FindDoctor(patient_id: String) -> Element {
    let mut doctor_type = use_signal(|| "General Physician".to_string());
    let mut location = use_signal(|| String::new());

    let _geolocator = init_geolocator(PowerMode::High);
    let coords = use_geolocation();

    let mut doctors = use_signal(Vec::new);
    let mut loading = use_signal(|| false);
    let mut error = use_signal(|| String::new());

    let mut searched_location = use_signal(|| String::new());

    let doctor_types = [
        "General Physician",
        "Cardiologist",
        "Dermatologist",
        "Neurologist",
        "Pediatrician",
        "Orthopedic",
        "Psychiatrist",
        "Gynecologist",
        "Dentist",
        "ENT",
        "Ophthalmologist",
    ];

    let search_doctors = move |_| {
        let selected_type = doctor_type();
        let selected_location = location();

        if selected_location.trim().is_empty() {
            error.set("Please enter a location.".to_string());
            return;
        }

        searched_location.set(selected_location.clone());

        error.set(String::new());
        doctors.set(Vec::new());
        loading.set(true);

        spawn(async move {
            match find_doctors(&selected_type, &selected_location).await {
                Ok(results) => {
                    doctors.set(results);
                }
                Err(_) => {
                    error.set(format!("Unable to find doctors."));
                }
            }

            loading.set(false);
        });
    };

    rsx! {
        Navbar { patient_id: patient_id.clone() }

        main { class: "relative flex min-h-[90vh] flex-col items-center px-6 py-6",

            div { class: "w-full max-w-3xl",

                // Header
                div { class: "mb-6",

                    h1 { class: "text-[32px] font-bold tracking-tight text-slate-900",
                        "Find Doctors 🧑‍⚕️"
                    }

                    p { class: "mt-2 mb-8 text-base text-slate-500",
                        "Find doctors and specialists near your location."
                    }
                }

                // Search form
                div { class: "rounded-2xl border border-slate-200 bg-white p-5 shadow-sm",

                    // Doctor type
                    div {
                        label { class: "mb-1.5 block text-sm font-semibold text-slate-700",
                            "Doctor"
                        }

                        select {
                            class: "w-full rounded-xl border border-slate-300 bg-white px-3 py-2.5 text-sm text-slate-700 outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-100",

                            value: "{doctor_type()}",

                            onchange: move |event| {
                                doctor_type.set(event.value());
                            },

                            for doctor in doctor_types {
                                option { value: "{doctor}", "{doctor}" }
                            }
                        }
                    }

                    // Location
                    div { class: "mt-4",

                        label { class: "mb-1.5 block text-sm font-semibold text-slate-700",
                            "Location"
                        }

                        div { class: "relative",

                            input {
                                class: "w-full rounded-xl border border-slate-300 px-3 py-2.5 pr-12 text-sm text-slate-700 outline-none placeholder:text-slate-400 focus:border-blue-500 focus:ring-2 focus:ring-blue-100",

                                r#type: "text",
                                placeholder: "e.g. Bengaluru, India",

                                value: "{location()}",

                                oninput: move |event| {
                                    location.set(event.value());
                                },
                            }

                            button {
                                r#type: "button",

                                class: "absolute right-2 top-1/2 -translate-y-1/2 rounded-lg p-2 text-slate-400 transition hover:text-blue-600 text-3xl font-semibold",

                                title: "Use my location",

                                onclick: move |_| {
                                    if let Ok(coords) = coords() {
                                        spawn(async move {
                                            if let Ok(place) = reverse_geocode(coords.latitude, coords.longitude)
                                                .await
                                            {
                                                location.set(place);
                                            }
                                        });
                                    }
                                },

                                "⌖"
                            }
                        }
                    }

                    // Search button
                    button {
                        r#type: "button",

                        class: "mt-5 flex w-full items-center justify-center gap-2 rounded-xl bg-blue-600 px-4 py-2.5 text-sm font-medium text-white transition hover:bg-blue-700 disabled:cursor-not-allowed disabled:opacity-60",

                        disabled: loading(),

                        onclick: search_doctors,

                        if loading() {
                            "Finding"
                            div { class: "h-4 w-4 animate-spin rounded-full border-2 border-white/40 border-t-white" }
                        } else {
                            "Find Doctors"
                        }
                    }

                    // Error
                    if !error().is_empty() {
                        div { class: "mt-4 rounded-xl bg-red-50 px-4 py-3 text-sm text-red-600",
                            "{error()}"
                        }
                    }
                }

                // Results
                if !searched_location().is_empty() && !loading() && !doctors().is_empty() {
                    div { class: "mt-8",

                        div { class: "mb-4 flex items-center justify-between",

                            span { class: "text-lg font-semibold text-slate-500",

                                "Doctors near: 📍 "

                                span { class: "text-slate-900", "{searched_location()}" }
                            }

                            span { class: "text-sm text-slate-500", "{doctors().len()} results" }
                        }

                        div { class: "space-y-4",

                            for doctor in doctors() {
                                div { class: "rounded-2xl border border-slate-200 bg-white p-5 shadow-sm",

                                    // Doctor name + specialty
                                    div { class: "flex items-start justify-between gap-4",

                                        div {
                                            h4 { class: "text-base font-semibold text-slate-900",
                                                "{doctor.doctor_name}"
                                            }

                                            if !doctor.specialty.is_empty() {
                                                p { class: "mt-0.5 text-sm text-blue-600",
                                                    "{doctor.specialty}"
                                                }
                                            }
                                        }
                                    }

                                    // Clinic + location
                                    if !doctor.clinic_or_hospital.is_empty() || !doctor.location.is_empty() {
                                        div { class: "mt-3 flex items-start gap-2 text-sm text-slate-500",

                                            div {
                                                if !doctor.clinic_or_hospital.is_empty() {
                                                    p { class: "font-medium text-slate-700",
                                                        "🏥 {doctor.clinic_or_hospital}"
                                                    }
                                                }

                                                if !doctor.location.is_empty() {
                                                    p { class: "mt-0.5", "📍 {doctor.location}" }
                                                }
                                            }
                                        }
                                    }

                                    // Details
                                    if !doctor.experience_years.is_empty() || !doctor.consultation_fee.is_empty() {
                                        div { class: "mt-4 flex flex-wrap gap-2",

                                            if !doctor.experience_years.is_empty() {
                                                span { class: "rounded-full bg-slate-100 px-3 py-1 text-sm font-medium text-slate-600",
                                                    "🩺 {doctor.experience_years} experience"
                                                }
                                            }

                                            if !doctor.consultation_fee.is_empty() {
                                                span { class: "rounded-full bg-green-50 px-3 py-1 text-sm font-medium text-green-800",
                                                    "💵 {doctor.consultation_fee}"
                                                }
                                            }
                                        }
                                    }

                                    // Appointment
                                    if !doctor.appointment_url.is_empty() {
                                        div { class: "mt-5",

                                            a {
                                                href: "{doctor.appointment_url}",
                                                target: "_blank",

                                                class: "inline-flex w-full items-center justify-center rounded-xl bg-blue-100 px-4 py-2.5 text-sm font-medium text-blue-600 transition hover:bg-blue-200",

                                                "Book Appointment ↗"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // No results
                if !loading() && error().is_empty() && !searched_location().is_empty()
                    && doctors().is_empty()
                {
                    div { class: "mt-8 rounded-2xl border border-dashed border-slate-300 bg-slate-50 px-6 py-10 text-center",

                        p { class: "text-sm font-medium text-slate-700", "No doctors found" }

                        p { class: "mt-1 text-sm text-slate-500",
                            "Try a different location or doctor type."
                        }
                    }
                }
            }
        }
    }
}
