use app_state::AppState;
use axum::{
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    serve::Serve,
    Json, Router,
};
use domain::AuthAPIError;
use redis::{Client, RedisResult};
use routes::{login, logout, signup, verify_2fa, verify_token};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::error::Error;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use utils::tracing::{make_span_with_request_id, on_request, on_response};

pub mod app_state;
pub mod domain {
    pub mod data_stores;
    pub mod email;
    pub mod email_client;
    pub mod error;
    pub mod password;
    pub mod user;
    // re-export the modules
    pub use data_stores::*;
    pub use email::*;
    pub use email_client::*;
    pub use error::*;
    pub use password::*;
    pub use user::*;
}
pub mod routes {
    pub mod login;
    pub mod logout;
    pub mod signup;
    pub mod verify_2fa;
    pub mod verify_token;
    // re-export the modules
    pub use login::*;
    pub use logout::*;
    pub use signup::*;
    pub use verify_2fa::*;
    pub use verify_token::*;
}
pub mod services {
    pub mod data_stores {
        pub mod hashmap_two_fa_code_store;
        pub mod hashmap_user_store;
        pub mod hashset_banned_token_store;
        pub mod postgres_user_store;
        pub mod redis_banned_token_store;
        pub mod redis_two_fa_code_store;
        // re-export the modules
        pub use hashmap_two_fa_code_store::*;
        pub use hashmap_user_store::*;
        pub use hashset_banned_token_store::*;
        pub use postgres_user_store::*;
        pub use redis_banned_token_store::*;
        pub use redis_two_fa_code_store::*;
    }
    pub mod mock_email_client;
}

pub mod utils {
    pub mod auth;
    pub mod constants;
    pub mod tracing;
}

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        // Allow the app service to call the auth service
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://147.182.208.125:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST])
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_token))
            .with_state(app_state)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        // Start the server and return the result
        tracing::info!("Listening on {}", self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing auth token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid auth token"),
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

pub async fn get_postgres_pool(url: &str) -> Result<PgPool, sqlx::Error> {
    // Create a new Postgres connection pool
    PgPoolOptions::new().max_connections(5).connect(url).await
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}/", redis_hostname);
    redis::Client::open(redis_url)
}
