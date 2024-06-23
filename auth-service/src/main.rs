#![allow(unused_variables)]

use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState,
    get_postgres_pool,
    services::{
        data_stores::{HashMapTwoFACodeStore, HashSetBannedTokenStore, PostgresUserStore},
        mock_email_client::MockEmailClient,
    },
    utils::constants::{prod, DATABASE_URL},
    Application,
};

#[tokio::main]
async fn main() {
    let pg_pool = configure_postgresql().await;
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let banned_token_store = Arc::new(RwLock::new(HashSetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashMapTwoFACodeStore::default()));
    let email_client = Arc::new(MockEmailClient);
    let app_state = AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create pool!");

    // Run the migrations
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations!");

    pg_pool
}
