use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<String, User>,
}

impl HashMapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return an error if the user already exists
        if self.users.contains_key(&user.email) {
            Err(UserStoreError::UserAlreadyExists)
        } else {
            self.users.insert(user.email.clone(), user);
            Ok(())
        }
    }

    pub async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
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
        let user = User::new("test@test.com".to_string(), "password".to_string(), false);

        let result = user_store.add_user(user);

        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn add_user_already_exists() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new("test@test.com".to_string(), "password".to_string(), false);
        let _ = user_store.add_user(user.clone());

        let result = user_store.add_user(user);

        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn get_user() {
        let mut user_store = HashMapUserStore::default();
        let user = User::new("test@test.com".to_string(), "password".to_string(), false);

        let _ = user_store.add_user(user.clone());

        let result = user_store.get_user(&user.email).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_user_not_found() {
        let user_store = HashMapUserStore::default();

        let result = user_store.get_user("test@test.com").await;

        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
