#![allow(dead_code, unused_variables)]

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
};

use super::SignupRequest;

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(body.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(body.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.read().await;

    // Return AuthAPIError::IncorrectCredentials if email doesn't exist in user_store
    if user_store.get_user(&email).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    // Return AuthAPIError::IncorrectCredentials if validation fails
    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentials);
    }

    Ok(StatusCode::OK)
}
