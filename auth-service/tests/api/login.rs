use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME,
};
use secrecy::{ExposeSecret, Secret};
use test_helpers::api_test;

#[api_test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    // Created user
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let two_fa_code_store = app.two_fa_code_store.read().await;

    let code_tuple = two_fa_code_store
        .get_code(&Email::parse(Secret::new(random_email)).unwrap())
        .await
        .expect("Failed to get 2FA code");

    assert_eq!(
        &json_body.login_attempt_id,
        code_tuple.0.as_ref().expose_secret()
    );
}

#[api_test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    // Created user
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[api_test]
async fn should_return_401_if_incorrect_credentials() {
    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    // Created user
    assert_eq!(response.status().as_u16(), 201);

    // Test for wrong email
    let login_body = serde_json::json!({
        "email": "test@test.com",
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(
        response.status().as_u16(),
        401,
        "Failed for input: {:?}",
        login_body
    );

    // Test for wrong password
    let login_body = serde_json::json!({
        "email": random_email,
        "password": "wrongpassword",
        "requires2FA": true
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(
        response.status().as_u16(),
        401,
        "Failed for input: {:?}",
        login_body
    );
}

#[api_test]
async fn should_return_400_if_invalid_input() {
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({"email": random_email, "password": "", "requires2FA": true }),
        serde_json::json!({"email": "", "password": "password123", "requires2FA": true }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[api_test]
async fn should_return_422_if_malformed_credentials() {
    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({"password": "password123", "requires2FA": true }),
        serde_json::json!({"email": random_email, "requires2FA": true }),
        serde_json::json!({}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_login(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
