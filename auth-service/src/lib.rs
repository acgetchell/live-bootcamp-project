use axum::{serve::Serve, Router};
use std::error::Error;
use tower_http::services::ServeDir;

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new().nest_service("/", ServeDir::new("assets"));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        // Start the server and return the result
        println!("Listening on {}", &self.address);
        self.server.await
    }
}
