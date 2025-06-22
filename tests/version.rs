const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let args: Vec<String> = std::env::args().collect();

    let fetch_remote_url: Option<String> = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--endpoint=").map(|s| s.to_string()));

    let current_version: Option<String> = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--version=").map(|s| s.to_string()));

    let fetch_remote_url = match fetch_remote_url {
        Some(url) if !url.is_empty() => url,
        _ => match std::env::var("DD_DRAGON_ENDPOINT") {
            Ok(url) if !url.is_empty() => url,
            _ => {
                panic!(
                    "{RED}ERROR: Remote URL is required but not provided. Use --fetch-remote= or set DD_DRAGON_ENDPOINT environment variable{RESET}"
                );
            }
        },
    };

    let current_version = match current_version {
        Some(version) if !version.is_empty() => version,
        _ => match std::env::var("LOL_VERSION") {
            Ok(version) if !version.is_empty() => version,
            _ => {
                panic!(
                    "{RED}ERROR: Current version is required but not provided. Use --version= or set LOL_VERSION environment variable{RESET}"
                );
            }
        },
    };

    let latest_version =
        match tutorlolv2::setup::api::fetch_version(reqwest::Client::new(), fetch_remote_url).await
        {
            Ok(version) => version,
            Err(e) => {
                panic!(
                    "{RED}ERROR: Failed to fetch latest version from API: {:#?}{RESET}",
                    e
                );
            }
        };

    println!("Latest version: {}", latest_version);

    if current_version != latest_version {
        panic!(
            "{RED}ERROR: APP OUTDATED{RESET}\nLatest version: {}",
            latest_version
        );
    }

    println!("{GREEN}✓ Version check passed. App is up to date.{RESET}");
}
