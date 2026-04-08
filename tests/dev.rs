use tutorlolv2_dev::HTTP_CLIENT;

#[tokio::test]
async fn scraper() {
    dotenvy::dotenv().unwrap();
    if let Err(e) = HTTP_CLIENT.call_scraper().await {
        println!("[!error] {e}");
    }
}
