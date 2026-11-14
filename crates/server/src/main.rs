#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    server::run().await
}
