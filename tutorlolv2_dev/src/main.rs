use tutorlolv2_dev::{HTTP_CLIENT, MayFail};

#[tokio::main]
async fn main() -> MayFail {
    dotenvy::dotenv().expect("Failed to read .env file");
    std::env::set_current_dir("../")?;

    HTTP_CLIENT.update_riot_cache().await?;
    HTTP_CLIENT.download_arts_img().await?;
    HTTP_CLIENT.download_items_img().await?;
    HTTP_CLIENT.download_runes_img().await?;
    HTTP_CLIENT.download_general_img().await

    // let _ = HTTP_CLIENT.call_scraper().await;
    // let _ = HTTP_CLIENT.combo_scraper().await;
}
