use std::{str::FromStr, sync::Arc};

use auth_service::{
    app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType},
    domain::Email,
    get_postgres_pool, get_redis_client,
    services::{
        data_stores::{PostgresUserStore, RedisBannedTokenStore, RedisTwoFACodeStore},
        postmark_email_client::PostmarkEmailClient,
    },
    utils::constants::{test, DATABASE_URL, REDIS_HOST_NAME},
    Application,
};
use reqwest::{cookie::Jar, Client};
use secrecy::{ExposeSecret, Secret};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Connection, Executor, PgConnection, PgPool,
};
use tokio::sync::RwLock;
use uuid::Uuid;
use wiremock::MockServer;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub http_client: reqwest::Client,
    pub email_server: MockServer,
    pub db_name: String,
    pub clean_up_called: bool,
}

impl TestApp {
    pub async fn new() -> Self {
        let db_name = Uuid::new_v4().to_string();
        let pg_pool = configure_postgresql(&db_name).await;
        let redis_connection = Arc::new(RwLock::new(configure_redis()));

        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
        let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
            redis_connection.clone(),
        )));
        let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));

        // Setup a mock email server
        let email_server = MockServer::start().await;
        let base_url = email_server.uri();
        let email_client = Arc::new(configure_postmark_email_client(base_url));

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client,
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        // Create new `TestApp` instance and return it
        Self {
            address,
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
            http_client,
            email_server,
            db_name,
            clean_up_called: false,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_login<Body>(&self, body: Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn clean_up(&mut self) {
        if self.clean_up_called {
            return;
        }

        delete_database(&self.db_name).await;
        self.clean_up_called = true;
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!("TestApp::clean_up was not called before dropping TestApp")
        }
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

async fn configure_postgresql(db_name: &str) -> PgPool {
    let postgres_conn_url = DATABASE_URL.to_owned();

    configure_database(&postgres_conn_url, db_name).await;

    let postgresql_conn_url_with_db =
        Secret::new(format!("{}/{}", postgres_conn_url.expose_secret(), db_name));

    // Create new connection pool and return it
    get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!")
}

async fn configure_database(db_conn_string: &Secret<String>, db_name: &str) {
    // Create database connection
    let connection = PgPoolOptions::new()
        .connect(db_conn_string.expose_secret())
        .await
        .expect("Failed to create Postgres connection pool!");

    // Create the database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database!");

    // Connect to the new database
    let db_conn_string = format! {"{}/{}", db_conn_string.expose_secret(), db_name};

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run the migrations
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database!");
}

async fn delete_database(db_name: &str) {
    let postgres_conn_url = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(postgres_conn_url.expose_secret())
        .expect("Failed to parse PostgreSQL connection string!");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to PostgreSQL!");

    // Kill any active connections to the database
    connection
        .execute(
            format!(
                r#"SELECT pg_terminate_backend(pg_stat_activity.pid)
            FROM pg_stat_activity
            WHERE pg_stat_activity.datname = '{}'
            AND pid <> pg_backend_pid();"#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to kill active connections!");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop database!");
}

fn configure_redis() -> redis::Connection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client!")
        .get_connection()
        .expect("Failed to get Redis connection!")
}

fn configure_postmark_email_client(base_url: String) -> PostmarkEmailClient {
    let postmark_auth_token = Secret::new("auth_token".to_owned());

    let sender = Email::parse(Secret::new(test::email_client::SENDER.to_owned())).unwrap();

    let http_client = Client::builder()
        .timeout(test::email_client::TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    PostmarkEmailClient::new(base_url, sender, postmark_auth_token, http_client)
}
