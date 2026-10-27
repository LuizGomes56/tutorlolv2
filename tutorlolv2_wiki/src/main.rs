use clap::{Parser, Subcommand, ValueEnum};
use tutorlolv2_wiki::{HTTP_CLIENT, LOL_VERSION, MayFail, champions, items, runes, scraper, stats};

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
    #[command(alias = "a")]
    Avif,
    #[command(alias = "f")]
    Fetch { function: Fetch },
    #[command(alias = "w")]
    Wiki { function: Wiki },
    /// Computes recommended items and runes from high elo ranked matches
    #[command(alias = "s")]
    Stats {
        #[arg(default_value = "all")]
        function: Stats,
        /// Most recent matches downloaded from each platform
        #[arg(short, long, default_value_t = stats::DEFAULT_MATCHES)]
        matches: usize,
        /// Comma separated platforms whose ladders are sampled, such as `kr,euw1,na1`
        #[arg(short, long, value_delimiter = ',', default_values = stats::DEFAULT_PLATFORMS)]
        platforms: Vec<String>,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Stats {
    #[clap(alias = "a")]
    All,
    #[clap(alias = "d")]
    Download,
    #[clap(alias = "p")]
    Parse,
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
    #[clap(alias = "s")]
    Scraper,
    #[clap(alias = "sd")]
    ScraperDownload,
    #[clap(alias = "sp")]
    ScraperParse,
    #[clap(alias = "sc")]
    ScraperConcat,
}

#[tokio::main]
async fn main() -> MayFail {
    let Cli { args } = Cli::parse();

    std::env::set_current_dir("../")?;

    match args {
        AppArgs::Update => {
            tutorlolv2_wiki::run().await?;

            HTTP_CLIENT.update_riot_cache().await?;
            HTTP_CLIENT.download_arts_img().await?;
            HTTP_CLIENT.download_items_img().await?;
            HTTP_CLIENT.download_runes_img().await?;
            HTTP_CLIENT.download_general_img().await?;

            #[cfg(feature = "avif")]
            tutorlolv2_wiki::avif::img_convert_avif();

            // scraper::run().await?;
        }
        AppArgs::Avif => {
            #[cfg(feature = "avif")]
            tutorlolv2_wiki::avif::img_convert_avif();
        }
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
                match gamev == LOL_VERSION {
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
            Wiki::Scraper => scraper::run().await,
            Wiki::ScraperDownload => scraper::download().await,
            Wiki::ScraperParse => scraper::parse(),
            Wiki::ScraperConcat => scraper::concat(),
        }
        .map_err(|e| format!("[wiki] Error: {e:?}"))?,
        AppArgs::Stats {
            function,
            matches,
            platforms,
        } => {
            let config = stats::StatsConfig { platforms, matches };

            match function {
                Stats::All => stats::run(&config).await,
                Stats::Download => stats::download(&config).await,
                Stats::Parse => stats::parse(),
            }
            .map_err(|e| format!("[stats] Error: {e:?}"))?
        }
    }

    Ok(())
}
