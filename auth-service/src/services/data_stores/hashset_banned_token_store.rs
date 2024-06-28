use std::collections::HashSet;

use secrecy::{ExposeSecret, Secret};

use crate::domain::{BannedTokenStore, BannedTokenStoreError};

#[derive(Default)]
pub struct HashSetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

#[async_trait::async_trait]
impl BannedTokenStore for HashSetBannedTokenStore {
    async fn add_banned_token(
        &mut self,
        token: Secret<String>,
    ) -> Result<(), BannedTokenStoreError> {
        self.banned_tokens.insert(token.expose_secret().to_owned());
        Ok(())
    }

    async fn is_banned(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        Ok(self.banned_tokens.contains(token.expose_secret()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_banned_token() {
        let mut banned_token_store = HashSetBannedTokenStore::default();

        // Test adding a new banned token
        let token = Secret::new("token".to_owned());

        let result = banned_token_store.add_banned_token(token.clone()).await;

        assert!(result.is_ok());
        assert!(banned_token_store
            .banned_tokens
            .contains(token.expose_secret()));
    }

    #[tokio::test]
    async fn is_banned() {
        let mut banned_token_store = HashSetBannedTokenStore::default();
        let token = Secret::new("token".to_owned());
        banned_token_store
            .banned_tokens
            .insert(token.expose_secret().to_owned());

        let result = banned_token_store.is_banned(&token).await;

        assert!(result.unwrap());
    }
}
