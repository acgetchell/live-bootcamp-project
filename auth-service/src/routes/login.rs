#![allow(dead_code, unused_variables)]

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, Password},
};

use super::SignupRequest;

// pub async fn login() -> impl IntoResponse {
//     StatusCode::OK.into_response()
// }

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = Email::parse(body.email.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(body.password.clone()).map_err(|_| AuthAPIError::InvalidCredentials)?;

    Ok(StatusCode::OK)
}
