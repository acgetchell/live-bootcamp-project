use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState,
    services::{
        hashmap_user_store::HashMapUserStore, HashMapTwoFACodeStore, HashSetBannedTokenStore,
    },
    utils::constants::prod,
    Application,
};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashMapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashSetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashMapTwoFACodeStore::default()));
    let app_state = AppState::new(user_store, banned_token_store, two_fa_code_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
