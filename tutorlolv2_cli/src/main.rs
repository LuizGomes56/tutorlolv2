use tutorlolv2_wiki::MayFail;

#[tokio::main]
async fn main() -> MayFail {
    std::env::set_current_dir("../")?;
    tutorlolv2_cli::run().await
}
