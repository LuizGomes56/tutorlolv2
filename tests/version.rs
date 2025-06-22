const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let args = std::env::args().collect::<Vec<String>>();

    let get_flag = |flag, fallback| {
        let maybe_flag = args
            .iter()
            .find_map(|arg| arg.strip_prefix(flag).map(|s| s.to_string()));
        match maybe_flag {
            Some(flag) if !flag.is_empty() => flag,
            _ => match std::env::var(fallback) {
                Ok(flag) if !flag.is_empty() => flag,
                _ => panic!("{RED}ERROR: {flag} is required but not provided{RESET}"),
            },
        }
    };

    let fetch_remote_url = get_flag("--fetch-remote=", "DD_DRAGON_ENDPOINT");
    let current_version = get_flag("--version=", "LOL_VERSION");

    let latest_version = match tutorlolv2::setup::api::fetch_version(
        reqwest::Client::new(),
        &fetch_remote_url,
    )
    .await
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
