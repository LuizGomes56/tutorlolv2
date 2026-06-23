#[tokio::main]
async fn main() {
    std::env::set_current_dir("../").unwrap();
    tutorlolv2_wiki::run().await.unwrap();
}
