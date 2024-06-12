use crate::helpers::{get_random_email, TestApp};
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

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

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({"password": "password123", "requires2FA": true }),
        serde_json::json!({"email": random_email, "requires2FA": true }),
        serde_json::json!({"email": random_email, "password": "password123" }),
        serde_json::json!({}),
        // This doesn't fail, and is there to show we return a 200 from Status::OK using the correct post_login helper function
        // serde_json::json!({"email": random_email, "password": "password123", "requires2FA": true }),
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
