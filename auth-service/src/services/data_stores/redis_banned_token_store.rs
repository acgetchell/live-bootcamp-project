use std::sync::Arc;

use color_eyre::eyre::Context;
use redis::{Commands, Connection};
use secrecy::{ExposeSecret, Secret};
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
    #[tracing::instrument(name = "Storing banned JWT in Redis", skip_all)]
    async fn add_banned_token(
        &mut self,
        token: Secret<String>,
    ) -> Result<(), BannedTokenStoreError> {
        let token_key = get_key(&token.expose_secret());

        let value = true;

        let ttl: u64 = TOKEN_TTL_SECONDS
            .try_into()
            .wrap_err("Failed to cast TOKEN_TTL_SECONDS to u64")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        let _: () = self
            .connection
            .write()
            .await
            .set_ex(token_key, value, ttl)
            .wrap_err("Failed to set banned token in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(())
    }
    #[tracing::instrument(name = "Checking if JWT is banned in Redis", skip_all)]
    async fn is_banned(&self, token: &Secret<String>) -> Result<bool, BannedTokenStoreError> {
        let token_key = get_key(token.expose_secret());

        let is_banned: bool = self
            .connection
            .write()
            .await
            .exists(&token_key)
            .wrap_err("Failed to check if token exists in Redis")
            .map_err(BannedTokenStoreError::UnexpectedError)?;

        Ok(is_banned)
    }
}

// We are using a key prefix to prevent collisions with other keys in the Redis database
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

pub fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
