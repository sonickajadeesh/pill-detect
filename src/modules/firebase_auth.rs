use gloo_storage::{LocalStorage, Storage};
use reqwest::Client;
use serde::{Deserialize, Serialize};

const SESSION_KEY: &str = "Pill Detect";
const FIREBASE_API_KEY: &str = env!("FIREBASE_API_KEY");

const SIGN_UP_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts:signUp";
const SIGN_IN_URL: &str = "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword";

#[derive(Clone, Debug, PartialEq)]
pub struct AuthUser {
    pub uid: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AuthSession {
    uid: String,
    email: String,
    id_token: String,
    refresh_token: String,
}

#[derive(Clone, Debug, Deserialize)]
struct FirebaseAuthResponse {
    #[serde(rename = "localId")]
    local_id: String,

    email: String,

    #[serde(rename = "idToken")]
    id_token: String,

    #[serde(rename = "refreshToken")]
    refresh_token: String,
}

#[derive(Clone, Debug, Deserialize)]
struct FirebaseErrorResponse {
    error: FirebaseError,
}

#[derive(Clone, Debug, Deserialize)]
struct FirebaseError {
    message: String,
}

#[derive(Clone, Debug)]
pub enum AuthError {
    Firebase(String),
    Network(String),
    Storage(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Firebase(message) => {
                write!(f, "{message}")
            }

            Self::Network(message) => {
                write!(f, "Network error: {message}")
            }

            Self::Storage(message) => {
                write!(f, "Storage error: {message}")
            }
        }
    }
}

impl std::error::Error for AuthError {}

#[derive(Clone)]
pub struct AuthService {
    client: Client,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> Result<AuthUser, AuthError> {
        let response = self
            .client
            .post(SIGN_UP_URL)
            .query(&[("key", FIREBASE_API_KEY)])
            .json(&serde_json::json!({
                "email": email,
                "password": password,
                "returnSecureToken": true,
            }))
            .send()
            .await
            .map_err(|error| AuthError::Network(error.to_string()))?;

        self.handle_auth_response(response).await
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<AuthUser, AuthError> {
        let response = self
            .client
            .post(SIGN_IN_URL)
            .query(&[("key", FIREBASE_API_KEY)])
            .json(&serde_json::json!({
                "email": email,
                "password": password,
                "returnSecureToken": true,
            }))
            .send()
            .await
            .map_err(|error| AuthError::Network(error.to_string()))?;

        self.handle_auth_response(response).await
    }

    pub fn current_user(&self) -> Result<Option<AuthUser>, AuthError> {
        let session: AuthSession = match LocalStorage::get(SESSION_KEY) {
            Ok(session) => session,
            Err(_) => return Ok(None),
        };

        Ok(Some(AuthUser {
            uid: session.uid,
            email: session.email,
        }))
    }

    async fn handle_auth_response(
        &self,
        response: reqwest::Response,
    ) -> Result<AuthUser, AuthError> {
        if !response.status().is_success() {
            return Err(parse_firebase_error(response).await);
        }

        let firebase_response: FirebaseAuthResponse = response
            .json()
            .await
            .map_err(|error| AuthError::Network(error.to_string()))?;

        let session = AuthSession {
            uid: firebase_response.local_id.clone(),
            email: firebase_response.email.clone(),
            id_token: firebase_response.id_token,
            refresh_token: firebase_response.refresh_token,
        };

        LocalStorage::set(SESSION_KEY, &session)
            .map_err(|error| AuthError::Storage(error.to_string()))?;

        Ok(AuthUser {
            uid: firebase_response.local_id,
            email: firebase_response.email,
        })
    }
}

async fn parse_firebase_error(response: reqwest::Response) -> AuthError {
    match response.json::<FirebaseErrorResponse>().await {
        Ok(error) => AuthError::Firebase(firebase_error_message(&error.error.message)),
        Err(error) => AuthError::Network(error.to_string()),
    }
}

fn firebase_error_message(message: &str) -> String {
    match message {
        "EMAIL_EXISTS" => "An account with this email already exists.".into(),
        "EMAIL_NOT_FOUND" => "No account exists with this email.".into(),
        "INVALID_PASSWORD" => "Incorrect password.".into(),
        "INVALID_LOGIN_CREDENTIALS" => "Invalid email or password.".into(),
        "USER_DISABLED" => "This account has been disabled.".into(),
        "WEAK_PASSWORD" => "Password must be at least 6 characters.".into(),
        "TOO_MANY_ATTEMPTS_TRY_LATER" => "Too many attempts. Please try again later.".into(),
        "OPERATION_NOT_ALLOWED" => "Email/password authentication is not enabled.".into(),
        _ => message.to_string(),
    }
}
