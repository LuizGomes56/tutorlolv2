#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tutorlolv2_build::run().await.unwrap();
}
