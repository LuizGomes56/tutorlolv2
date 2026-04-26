use reqwest::{
    Client,
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT},
};
use std::{fmt::Display, io::Write, path::Path, sync::LazyLock};

pub type MayFail<T = (), E = Box<dyn core::error::Error + Send + Sync>> = Result<T, E>;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let mut headers = HeaderMap::new();

    for (key, val) in [
        (
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
        ),
        (ACCEPT, "application/json, text/plain, */*"),
        (ACCEPT_LANGUAGE, "en-US,en;q=0.9"),
        (ACCEPT_ENCODING, "gzip, deflate"),
    ] {
        headers.insert(key, HeaderValue::from_static(val));
    }

    reqwest::Client::builder()
        .default_headers(headers)
        .cookie_store(true)
        .build()
        .unwrap()
});

pub async fn fetch(save_to: impl AsRef<Path>, link: impl Display) -> MayFail<String> {
    let path = save_to.as_ref();

    if std::fs::exists(path)? {
        println!("[exists] {path:?}");
        return Ok(std::fs::read_to_string(path)?);
    }

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    let url = format!("https://wiki.leagueoflegends.com/en-us/{link}");

    async fn request(url: &str) -> MayFail<String> {
        Ok(CLIENT.get(url).send().await?.text().await?)
    }

    let mut bytes = request(&url).await?;

    if bytes.contains("Too many requests") {
        eprintln!("[error] Too many requests, trying again in 500ms");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        bytes = request(&url).await?;
    }

    println!("[fetch] {link} -> {path:?}");

    let mut file = std::fs::File::create(path)?;
    file.write_all(bytes.as_bytes())?;
    file.flush()?;

    Ok(bytes)
}
