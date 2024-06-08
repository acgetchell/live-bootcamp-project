use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashMapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return an error if the user already exists
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_user() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new(
            Email::parse("test@test.com".to_owned()).unwrap(),
            Password::parse("password".to_owned()).unwrap(),
            false,
        );

        // Test adding a new user
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // Test adding an existing user
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn get_user() {
        let mut user_store = HashMapUserStore::default();
        let email = Email::parse("test@test.com".to_string()).unwrap();
        let user = User::new(
            email,
            Password::parse("password".to_owned()).unwrap(),
            false,
        );
        user_store.add_user(user.clone()).await.unwrap();

        // Test getting a user that exists
        let result = user_store.get_user(&user.email).await;
        assert!(result.is_ok());

        // Test getting a user that doesn't exist
        let result = user_store
            .get_user(&Email::parse("nonexistant@test.com".to_owned()).unwrap())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn validate_user() {
        let mut user_store = HashMapUserStore::default();
        let email = Email::parse("test@test.com".to_owned()).unwrap();
        let password = Password::parse("password".to_owned()).unwrap();
        let user = User::new(email.clone(), password.clone(), false);
        user_store.add_user(user.clone()).await.unwrap();

        // Test validating a user with the correct password
        let result = user_store.validate_user(&email, &password).await;
        assert!(result.is_ok());

        // Test validating a user with the wrong password
        let wrong_password = Password::parse("wrong_password".to_owned()).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // Test validating a user that doesn't exist
        let result = user_store
            .validate_user(
                &Email::parse("nonexistant@test.com".to_owned()).unwrap(),
                &password,
            )
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
