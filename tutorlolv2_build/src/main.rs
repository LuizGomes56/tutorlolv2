#[tokio::main(flavor = "multi_thread")]
async fn main() {
    tutorlolv2_build::new_runner().await.unwrap();
}
