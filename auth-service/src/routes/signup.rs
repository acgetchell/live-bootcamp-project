#![allow(dead_code, unused_variables)]

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request` variable
    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = state.user_store.write().await;

    // Add the user to the user store
    user_store.add_user(user).unwrap();

    let response = Json(SignupResponse { message: "User created successfully!".to_string(),});
    
    (StatusCode::CREATED, response)
}

#[derive(Serialize)]
pub struct SignupResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct SignupRequest {
    email: String,
    password: String,
    #[serde(rename = "requires2FA")]
    requires_2fa: bool,
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    todo!()
}
