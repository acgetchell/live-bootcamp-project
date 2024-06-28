use std::collections::HashMap;

use crate::domain::{
    data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
    email::Email,
};

#[derive(Default)]
pub struct HashMapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

#[async_trait::async_trait]
impl TwoFACodeStore for HashMapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        match self.codes.remove(email) {
            Some(_) => Ok(()),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        match self.codes.get(email) {
            Some((login_attempt_id, code)) => Ok((login_attempt_id.clone(), code.clone())),
            None => Err(TwoFACodeStoreError::LoginAttemptIdNotFound),
        }
    }
}

#[cfg(test)]
mod tests {

    use secrecy::Secret;

    use super::*;

    #[tokio::test]
    async fn test_add_code() {
        let mut store = HashMapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("test@test.com".to_owned())).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        let result = store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await;

        assert_eq!(result, Ok(()));
        assert_eq!(store.codes.get(&email), Some(&(login_attempt_id, code)));
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashMapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("test@example.com".to_owned())).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();

        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.remove_code(&email).await;

        assert!(result.is_ok());
        assert_eq!(store.codes.get(&email), None);
    }

    #[tokio::test]
    async fn test_get_code() {
        let mut store = HashMapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("test@example.com".to_owned())).unwrap();
        let login_attempt_id = LoginAttemptId::default();
        let code = TwoFACode::default();
        store
            .codes
            .insert(email.clone(), (login_attempt_id.clone(), code.clone()));

        let result = store.get_code(&email).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (login_attempt_id, code));
    }

    #[tokio::test]
    async fn test_get_code_not_found() {
        let store = HashMapTwoFACodeStore::default();
        let email = Email::parse(Secret::new("test@example.com".to_owned())).unwrap();

        let result = store.get_code(&email).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            TwoFACodeStoreError::LoginAttemptIdNotFound
        );
    }
}
