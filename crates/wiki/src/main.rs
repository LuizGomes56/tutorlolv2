use clap::{Parser, Subcommand, ValueEnum};
use wiki::{
    HTTP_CLIENT, MayFail, champions, items, lock, lol_version, map_changes, runes, scraper, spells,
    stats, version, wikitext,
};

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
    /// Downloads and parses the summoner spell module used by Ignite.
    Spells,
    SpellsDownload,
    SpellsParse,
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
    /// Downloads raw ability sources for the structured champion model.
    #[clap(alias = "cda")]
    ChampionsDownloadAbilities,
    #[clap(alias = "cde")]
    ChampionsDownloadExceptions,
    #[clap(alias = "cpf")]
    ChampionsParseFull,
    #[clap(alias = "cpt")]
    ChampionsParseTemplates,
    #[clap(alias = "cpe")]
    ChampionsParseExceptions,
    /// Writes the formula-ready model of every ability to `cache/wiki/champions/model.json`
    #[clap(alias = "cpm")]
    ChampionsParseModel,

    #[clap(alias = "i")]
    Items,
    #[clap(alias = "id")]
    ItemsDownload,
    #[clap(alias = "ip")]
    ItemsParse,
    /// Downloads and parses the changes of the game modes in `map_changes::MAPS`
    #[clap(alias = "m")]
    MapChanges,
    #[clap(alias = "md")]
    MapChangesDownload,
    #[clap(alias = "mp")]
    MapChangesParse,
    /// Lists the changes of each game mode that may change a damage
    #[clap(alias = "mr")]
    MapChangesReport,
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
    #[clap(alias = "l")]
    Lock,
    /// Checks the generators against the lock file of the latest archived version
    #[clap(alias = "lc")]
    LockCheck,
}

#[tokio::main]
async fn main() -> MayFail {
    let Cli { args } = Cli::parse();

    std::env::set_current_dir(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap(),
    )?;

    match args {
        AppArgs::Update => {
            version::check().await?;
            wiki::run().await?;

            HTTP_CLIENT.update_riot_cache().await?;
            HTTP_CLIENT.download_arts_img().await?;
            HTTP_CLIENT.download_items_img().await?;
            HTTP_CLIENT.download_runes_img().await?;
            HTTP_CLIENT.download_general_img().await?;

            #[cfg(feature = "avif")]
            wiki::avif::img_convert_avif();

            // scraper::run().await?;
        }
        AppArgs::Avif => {
            #[cfg(feature = "avif")]
            wiki::avif::img_convert_avif();
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
                let current = lol_version();

                match gamev == current {
                    true => println!("App is up to date with game version"),
                    false => println!("App is outdated: Expected {gamev}, found: {current}"),
                }
            }
        },
        AppArgs::Wiki { function } => match function {
            Wiki::All => wiki::run().await,
            Wiki::Spells => spells::run().await,
            Wiki::SpellsDownload => spells::download().await,
            Wiki::SpellsParse => spells::parse(),
            Wiki::Champions => champions::run().await,
            Wiki::ChampionsConcat => champions::concat(),
            Wiki::ChampionsDownloadFull => champions::full::download().await.map(|_| ()),
            Wiki::ChampionsParseFull => champions::full::parse().map(|_| ()),
            Wiki::ChampionsDownloadTemplates => champions::template::download().await,
            Wiki::ChampionsParseTemplates => champions::template::parse(),
            Wiki::ChampionsDownloadAbilities => champions::download_ability_sources().await,
            Wiki::ChampionsDownloadExceptions => champions::exceptions::download().await,
            Wiki::ChampionsParseExceptions => champions::exceptions::parse(),
            Wiki::ChampionsParseModel => wikitext::build::parse_model(),
            Wiki::Items => items::run().await,
            Wiki::ItemsDownload => items::download().await.map(|_| ()),
            Wiki::ItemsParse => items::parse().and_then(|_| items::item_parser::parse_items()),
            Wiki::MapChanges => map_changes::run().await,
            Wiki::MapChangesDownload => map_changes::download().await,
            Wiki::MapChangesParse => map_changes::parse(),
            Wiki::MapChangesReport => map_changes::report(),
            Wiki::Runes => runes::run().await,
            Wiki::RunesLinks => runes::links().await,
            Wiki::RunesDownload => runes::download().await,
            Wiki::RunesParse => runes::parse(),
            Wiki::RunesConcat => runes::concat(),
            Wiki::Scraper => scraper::run().await,
            Wiki::ScraperDownload => scraper::download().await,
            Wiki::ScraperParse => scraper::parse(),
            Wiki::ScraperConcat => scraper::concat(),
            Wiki::Lock => lock::write(),
            Wiki::LockCheck => lock::check(),
        }
        .map_err(|e| format!("[wiki] Error: {e:?}"))?,
        AppArgs::Stats {
            function,
            matches,
            platforms,
        } => {
            let config = stats::StatsConfig { platforms, matches };

            match function {
                Stats::All => config.run().await,
                Stats::Download => config.download().await,
                Stats::Parse => stats::parse(),
            }
            .map_err(|e| format!("[stats] Error: {e:?}"))?
        }
    }

    Ok(())
}
