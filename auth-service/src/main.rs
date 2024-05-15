use auth_service::Application;

#[tokio::main]
async fn main() {
    let app = Application::build("0.0.0.0:2000")
        .await
        .expect("Failed to build app");
    app.run().await.expect("Failed to run app");
}
