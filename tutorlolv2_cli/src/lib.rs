use clap::{Parser, Subcommand, ValueEnum};
use std::{str::FromStr, sync::LazyLock};
use tutorlolv2_dev::{
    ENV_CONFIG, HTTP_CLIENT, MayFail,
    gen_factories::{Parser as _, wiki_champions::ChampionParser, wiki_items::ItemParser},
    update,
};
use tutorlolv2_gen::{ChampionId, ItemId};
use tutorlolv2_wiki::{champions, items, runes};

fn from_str_err<T>(s: &str, into: &str) -> Result<T, String> {
    Err(format!("Value {s:?} can't be converted into {into}"))
}

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub args: GenArgs,
}

#[derive(Clone, Copy, Debug)]
pub enum RunTarget {
    Champion(ChampionId),
    Item(ItemId),
    All,
}

impl FromStr for RunTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" | "a" => Ok(Self::All),
            s if let Ok(champion_id) = ChampionId::from_str(s) => Ok(Self::Champion(champion_id)),
            s if let Ok(item_id) = ItemId::from_str(s) => Ok(Self::Item(item_id)),
            _ => from_str_err(s, "ChampionId or ItemId"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GenCreator {
    All,
    Champion(ChampionId),
    Item(ItemId),
}

impl FromStr for GenCreator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" | "a" => Ok(Self::All),
            s if let Ok(champion_id) = ChampionId::from_str(s) => Ok(Self::Champion(champion_id)),
            s if let Ok(item_id) = ItemId::from_str(s) => Ok(Self::Item(item_id)),
            _ => from_str_err(s, "ChampionId or ItemId"),
        }
    }
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Fetch {
    #[clap(alias = "i")]
    Images,
    #[clap(alias = "c")]
    Cache,
    #[clap(alias = "s")]
    Scraper,
    #[clap(alias = "v")]
    Version,
}

#[derive(Subcommand, Debug)]
pub enum GenArgs {
    #[command(alias = "c")]
    Create { creator: GenCreator },
    #[command(alias = "r")]
    Run { target: RunTarget },
    #[command(alias = "p")]
    Progress,
    #[command(alias = "u")]
    Update,
    #[command(alias = "h")]
    Html,
    #[command(alias = "s")]
    Setup { setup: Setup },
    #[command(alias = "b")]
    Build,
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

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Setup {
    #[clap(alias = "i")]
    Items,
    #[clap(alias = "p")]
    Prettify,
    #[clap(alias = "f")]
    Folders,
}

static IPARSER: LazyLock<ItemParser> = LazyLock::new(|| ItemParser::new().unwrap());
static CPARSER: LazyLock<ChampionParser> = LazyLock::new(|| ChampionParser::new().unwrap());

pub async fn run() -> MayFail {
    let Cli { args } = Cli::parse();

    dotenvy::dotenv()?;
    std::env::set_current_dir("../")?;

    match args {
        GenArgs::Create { creator } => match creator {
            GenCreator::All => {
                CPARSER.create_all()?;
                IPARSER.create_all()?;
            }
            GenCreator::Champion(champion_id) => CPARSER.create(champion_id.debug())?,
            GenCreator::Item(item_id) => IPARSER.create(item_id.debug())?,
        },
        GenArgs::Run { target } => match target {
            RunTarget::Champion(champ) => CPARSER.run(champ.debug())?,
            RunTarget::Item(item) => IPARSER.run(item.debug())?,
            RunTarget::All => {
                CPARSER.run_all()?;
                IPARSER.run_all()?;
            }
        },
        GenArgs::Progress => CPARSER.progress(),
        GenArgs::Update => {
            update::setup_project_folders()?;
            CPARSER.create_all()?;
            CPARSER.run_all()?;
            IPARSER.create_all()?;
            IPARSER.run_all()?;
            std::env::set_current_dir("./tutorlolv2_build")?;
            tutorlolv2_build::run()?;
        }
        GenArgs::Html => tutorlolv2_html::run(),
        GenArgs::Setup { setup } => match setup {
            Setup::Items => {
                update::setup_damaging_items()?;
                update::setup_runes_json()?;
                update::setup_internal_items()?;
                update::prettify_internal_items()?;
            }
            Setup::Prettify => update::prettify_internal_items()?,
            Setup::Folders => update::setup_project_folders()?,
        },
        GenArgs::Build => {
            std::env::set_current_dir("./tutorlolv2_build")?;
            tutorlolv2_build::run()?;
        }
        GenArgs::Fetch { function } => match function {
            Fetch::Images => {
                HTTP_CLIENT.download_arts_img().await?;
                HTTP_CLIENT.download_items_img().await?;
                HTTP_CLIENT.download_runes_img().await?;
                HTTP_CLIENT.download_general_img().await?;
            }
            Fetch::Cache => {
                HTTP_CLIENT.update_riot_cache().await?;
                HTTP_CLIENT.update_language_cache().await?;
            }
            Fetch::Scraper => {
                HTTP_CLIENT.call_scraper().await?;
                HTTP_CLIENT.combo_scraper().await?;
            }
            Fetch::Version => {
                let gamev = HTTP_CLIENT.fetch_version().await?;
                let currv = &ENV_CONFIG.lol_version;
                match &gamev == currv {
                    true => println!("App is up to date with game version"),
                    false => println!("App is outdated: Expected {gamev}, found: {currv}"),
                }
            }
        },
        GenArgs::Wiki { function } => match function {
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
