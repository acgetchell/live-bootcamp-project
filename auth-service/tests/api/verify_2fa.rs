use auth_service::domain::LoginAttemptId;

use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();
    let login_attempt_id = LoginAttemptId::default().as_ref().to_owned();

    let test_cases = [
        serde_json::json!({"2FACode": "123456"}),
        serde_json::json!({"email": random_email}),
        serde_json::json!({"login_attempt_id": login_attempt_id}),
        serde_json::json!({"2FACode": "123456", "email": random_email}),
        serde_json::json!({"2FACode": "123456", "login_attempt_id": login_attempt_id}),
        serde_json::json!({"email": random_email, "login_attempt_id": login_attempt_id}),
        serde_json::json!({}),
    ];

    for test_case in test_cases {
        let response = app.post_verify_2fa(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
