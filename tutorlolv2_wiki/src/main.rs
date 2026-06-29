use tutorlolv2_wiki::client::MayFail;

#[tokio::main]
async fn main() -> MayFail {
    std::env::set_current_dir("../")?;
    tutorlolv2_wiki::run().await
}
