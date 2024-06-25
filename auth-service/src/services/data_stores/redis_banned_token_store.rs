use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    domain::data_stores::{BannedTokenStore, BannedTokenStoreError},
    utils::auth::TOKEN_TTL_SECONDS,
};

pub struct RedisBannedTokenStore {
    connection: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(connection: Arc<RwLock<Connection>>) -> Self {
        Self { connection }
    }
}

#[async_trait::async_trait]
impl BannedTokenStore for RedisBannedTokenStore {
    async fn add_banned_token(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        let token_key = get_key(&token);

        let value = true;

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        let _: () = self
            .connection
            .write()
            .await
            .set_ex(token_key, value, ttl)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn is_banned(&self, token: &str) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token);

        let is_banned: bool = self
            .connection
            .write()
            .await
            .exists(&token_key)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;

        Ok(is_banned)
    }
}

// We are using a key prefix to prevent collisions with other keys in the Redis database
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

pub fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
