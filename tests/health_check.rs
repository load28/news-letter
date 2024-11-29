#[tokio::test]
async fn health_check() {
    spawn_app().await.expect("Failed to spawn app");
}

async fn spawn_app() -> std::io::Result<()> {
    todo!()
}
