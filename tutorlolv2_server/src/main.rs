#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    tutorlolv2_server::run().await
}
