use news_letter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
}
