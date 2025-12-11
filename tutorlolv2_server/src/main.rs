use tutorlolv2_server::run;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::io::Result<()> {
    run().await
}
