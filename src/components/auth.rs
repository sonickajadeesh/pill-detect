use dioxus::prelude::*;

use crate::modules::firebase_auth::{AuthService, AuthUser};

#[derive(Props, Clone, PartialEq)]
pub struct AuthPageProps {
    pub on_authenticated: EventHandler<AuthUser>,
}

#[component]
pub fn AuthPage(props: AuthPageProps) -> Element {
    let mut is_sign_up = use_signal(|| false);
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut error = use_signal(|| Option::<String>::None);
    let mut success = use_signal(|| Option::<String>::None);

    let mut auth_action = use_action(move |(email, password, sign_up): (String, String, bool)| {
        let auth = AuthService::new();

        async move {
            if sign_up {
                auth.sign_up(&email, &password).await
            } else {
                auth.sign_in(&email, &password).await
            }
        }
    });

    let sign_up = *is_sign_up.read();
    let loading = auth_action.pending();

    use_effect(move || {
        if let Some(result) = auth_action.value() {
            match result {
                Ok(user) => {
                    if sign_up {
                        success.set(Some(
                            "Account created successfully. Please sign in.".to_string(),
                        ));

                        is_sign_up.set(false);
                        password.set(String::new());
                        confirm_password.set(String::new());
                        auth_action.reset();
                    } else {
                        props.on_authenticated.call(user.read().clone());
                    }
                }
                Err(_) => {}
            }
        }
    });

    rsx! {
        div { class: "min-h-[96vh] bg-slate-50 flex items-center justify-center px-4 py-12",

            div { class: "w-full max-w-md",

                h1 { class: "mb-5 text-center text-4xl font-semibold tracking-tight text-slate-900",
                    "🩺 Pill Detect"
                }

                div { class: "rounded-xl border border-slate-200 bg-white p-6 shadow-sm sm:p-8",

                    h2 { class: "mb-6 text-xl font-semibold text-slate-900",

                        if sign_up {
                            "Create your account"
                        } else {
                            "Sign in to continue"
                        }
                    }

                    if let Some(message) = success.read().as_ref() {
                        div { class: "mb-5 rounded-lg border border-green-200 bg-green-50 px-3 py-2.5 text-sm text-green-700",
                            "{message}"
                        }
                    }

                    form {
                        class: "space-y-5",

                        onsubmit: move |event| {
                            event.prevent_default();

                            let email_value =
                                email.read().trim().to_string();

                            let password_value =
                                password.read().clone();

                            if email_value.is_empty()
                                || password_value.is_empty()
                            {
                                error.set(Some("Please enter your email and password.".to_string()));
                                return;
                            }

                            if sign_up
                                && password_value
                                    != *confirm_password.read()
                            {
                                error.set(Some("Passwords do not match.".to_string()));
                                return;
                            }

                            error.set(None);
                            success.set(None);
                            auth_action.call((email_value, password_value, sign_up));
                        },

                        div { class: "space-y-1.5",

                            label {
                                r#for: "email",
                                class: "block text-sm font-medium text-slate-700",
                                "Email"
                            }

                            input {
                                id: "email",
                                r#type: "email",
                                required: true,
                                autocomplete: "email",
                                placeholder: "you@example.com",
                                value: "{email}",
                                class: "w-full rounded-lg border border-slate-300 bg-white px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-slate-500 focus:ring-2 focus:ring-slate-200",

                                oninput: move |event| {
                                    email.set(event.value());
                                },
                            }
                        }

                        div { class: "space-y-1.5",

                            label {
                                r#for: "password",
                                class: "block text-sm font-medium text-slate-700",
                                "Password"
                            }

                            input {
                                id: "password",
                                r#type: "password",
                                required: true,
                                autocomplete: if sign_up { "new-password" } else { "current-password" },
                                placeholder: "••••••••",
                                value: "{password}",
                                class: "w-full rounded-lg border border-slate-300 bg-white px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-slate-500 focus:ring-2 focus:ring-slate-200",

                                oninput: move |event| {
                                    password.set(event.value());
                                },
                            }
                        }

                        if sign_up {
                            div { class: "space-y-1.5",

                                label {
                                    r#for: "confirm-password",
                                    class: "block text-sm font-medium text-slate-700",
                                    "Confirm password"
                                }

                                input {
                                    id: "confirm-password",
                                    r#type: "password",
                                    required: true,
                                    autocomplete: "new-password",
                                    placeholder: "••••••••",
                                    value: "{confirm_password}",
                                    class: "w-full rounded-lg border border-slate-300 bg-white px-3.5 py-2.5 text-sm text-slate-900 outline-none transition placeholder:text-slate-400 focus:border-slate-500 focus:ring-2 focus:ring-slate-200",

                                    oninput: move |event| {
                                        confirm_password.set(event.value());
                                    },
                                }
                            }
                        }

                        if let Some(message) = error.read().as_ref() {
                            p { class: "text-sm text-red-600", "{message}" }
                        }

                        if let Some(Err(auth_error)) = auth_action.value() {
                            p { class: "text-sm text-red-600", "{auth_error}" }
                        }

                        button {
                            r#type: "submit",
                            disabled: loading,
                            class: "w-full rounded-lg bg-slate-900 px-4 py-2.5 text-sm font-medium text-white transition hover:bg-slate-800 focus:outline-none focus:ring-2 focus:ring-slate-300 focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",

                            if loading {
                                if sign_up {
                                    "Creating account..."
                                } else {
                                    "Signing in..."
                                }
                            } else if sign_up {
                                "Sign Up"
                            } else {
                                "Sign In"
                            }
                        }
                    }

                    div { class: "my-6 flex items-center gap-3",

                        div { class: "h-px flex-1 bg-slate-200" }

                        span { class: "text-xs text-slate-400", "OR" }

                        div { class: "h-px flex-1 bg-slate-200" }
                    }

                    div { class: "text-center text-sm text-slate-500",

                        if sign_up {
                            span { "Already have an account? " }

                            button {
                                r#type: "button",
                                class: "font-medium text-slate-900 underline underline-offset-4 hover:text-slate-600",

                                onclick: move |_| {
                                    is_sign_up.set(false);
                                    error.set(None);
                                    success.set(None);
                                    auth_action.reset();
                                },

                                "Sign in"
                            }
                        } else {
                            span { "Don't have an account? " }

                            button {
                                r#type: "button",
                                class: "font-medium text-slate-900 underline underline-offset-4 hover:text-slate-600",

                                onclick: move |_| {
                                    is_sign_up.set(true);
                                    error.set(None);
                                    success.set(None);
                                    auth_action.reset();
                                },

                                "Create account"
                            }
                        }
                    }
                }
            }
        }
    }
}
