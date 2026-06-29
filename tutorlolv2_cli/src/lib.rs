use clap::{Parser, Subcommand, ValueEnum};
use tutorlolv2_dev::{HTTP_CLIENT, LOL_VERSION, MayFail};
use tutorlolv2_wiki::{champions, items, runes};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub args: AppArgs,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Fetch {
    #[clap(alias = "i")]
    Images,
    #[clap(alias = "c")]
    Cache,
    #[clap(alias = "v")]
    Version,
}

#[derive(Subcommand, Debug)]
pub enum AppArgs {
    #[command(alias = "u")]
    Update,
    #[command(alias = "h")]
    Html,
    #[command(alias = "f")]
    Fetch { function: Fetch },
    #[command(alias = "w")]
    Wiki { function: Wiki },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Wiki {
    #[clap(alias = "a")]
    All,
    #[clap(alias = "c")]
    Champions,
    #[clap(alias = "cc")]
    ChampionsConcat,
    #[clap(alias = "cdf")]
    ChampionsDownloadFull,
    #[clap(alias = "cdt")]
    ChampionsDownloadTemplates,
    #[clap(alias = "cda")]
    ChampionsDownloadAbilities,
    #[clap(alias = "cpf")]
    ChampionsParseFull,
    #[clap(alias = "cpt")]
    ChampionsParseTemplates,
    #[clap(alias = "cpa")]
    ChampionsParseAbilities,
    #[clap(alias = "i")]
    Items,
    #[clap(alias = "id")]
    ItemsDownload,
    #[clap(alias = "ip")]
    ItemsParse,
    #[clap(alias = "r")]
    Runes,
    #[clap(alias = "rl")]
    RunesLinks,
    #[clap(alias = "rd")]
    RunesDownload,
    #[clap(alias = "rp")]
    RunesParse,
    #[clap(alias = "rc")]
    RunesConcat,
}

pub async fn run() -> MayFail {
    let Cli { args } = Cli::parse();

    match args {
        AppArgs::Update => {
            tutorlolv2_wiki::run().await?;

            HTTP_CLIENT.update_riot_cache().await?;
            HTTP_CLIENT.download_arts_img().await?;
            HTTP_CLIENT.download_items_img().await?;
            HTTP_CLIENT.download_runes_img().await?;
            HTTP_CLIENT.download_general_img().await?;

            // let _ = HTTP_CLIENT.call_scraper().await;
            // let _ = HTTP_CLIENT.combo_scraper().await;
        }
        AppArgs::Html => tutorlolv2_html::run(),
        AppArgs::Fetch { function } => match function {
            Fetch::Images => {
                HTTP_CLIENT.download_arts_img().await?;
                HTTP_CLIENT.download_items_img().await?;
                HTTP_CLIENT.download_runes_img().await?;
                HTTP_CLIENT.download_general_img().await?;
            }
            Fetch::Cache => HTTP_CLIENT.update_riot_cache().await?,
            Fetch::Version => {
                let gamev = HTTP_CLIENT.fetch_version().await?;
                match &gamev == LOL_VERSION {
                    true => println!("App is up to date with game version"),
                    false => println!("App is outdated: Expected {gamev}, found: {LOL_VERSION}"),
                }
            }
        },
        AppArgs::Wiki { function } => match function {
            Wiki::All => tutorlolv2_wiki::run().await,
            Wiki::Champions => champions::run().await,
            Wiki::ChampionsConcat => champions::concat(),
            Wiki::ChampionsDownloadFull => champions::full::download().await.map(|_| ()),
            Wiki::ChampionsParseFull => champions::full::parse().map(|_| ()),
            Wiki::ChampionsDownloadTemplates => champions::template::download().await,
            Wiki::ChampionsParseTemplates => champions::template::parse(),
            Wiki::ChampionsDownloadAbilities => champions::abilities::download().await,
            Wiki::ChampionsParseAbilities => champions::abilities::parse(),
            Wiki::Items => items::run().await,
            Wiki::ItemsDownload => items::download().await.map(|_| ()),
            Wiki::ItemsParse => items::parse().map(|_| ()),
            Wiki::Runes => runes::run().await,
            Wiki::RunesLinks => runes::links().await,
            Wiki::RunesDownload => runes::download().await,
            Wiki::RunesParse => runes::parse(),
            Wiki::RunesConcat => runes::concat(),
        }
        .map_err(|e| format!("[wiki] Error: {e:?}"))?,
    }

    Ok(())
}
