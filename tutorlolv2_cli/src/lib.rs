use clap::{Parser, Subcommand, ValueEnum};
use std::str::FromStr;
use tutorlolv2_dev::{
    ENV_CONFIG, HTTP_CLIENT, MayFail,
    gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory},
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
    Factory(fn()),
    All,
}

impl FromStr for RunTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" | "a" => Ok(Self::All),
            "items" | "i" => Ok(Self::Factory(ItemFactory::run_all)),
            "champions" | "c" => Ok(Self::Factory(ChampionFactory::run_all)),
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
}

impl FromStr for GenCreator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" | "a" => Ok(Self::All),
            s if let Ok(champion_id) = ChampionId::from_str(s) => Ok(Self::Champion(champion_id)),
            _ => from_str_err(s, "ChampionId"),
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

pub async fn run() -> MayFail {
    let Cli { args } = Cli::parse();

    dotenvy::dotenv()?;
    std::env::set_current_dir("../")?;

    match args {
        GenArgs::Create { creator } => match creator {
            GenCreator::All => ChampionFactory::create_all()?,
            GenCreator::Champion(champion_id) => ChampionFactory::create(champion_id.debug())?,
        },
        GenArgs::Run { target } => match target {
            RunTarget::Champion(champ) => {
                ChampionFactory::run(champ.debug())?;
            }
            RunTarget::Item(item) => {
                ItemFactory::run(item.debug(), item.to_riot_id())?;
            }
            RunTarget::Factory(f) => f(),
            RunTarget::All => {
                ChampionFactory::run_all();
                ItemFactory::run_all();
            }
        },
        GenArgs::Progress => ChampionFactory::progress(),
        GenArgs::Update => {
            update::setup_project_folders()?;
            ChampionFactory::create_all()?;
            ChampionFactory::run_all();
            ItemFactory::run_all();
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
                let riot_version = HTTP_CLIENT.fetch_version().await?;
                let curr_version = &ENV_CONFIG.lol_version;
                if &riot_version == curr_version {
                    println!("App is up to date with game version");
                } else {
                    println!("App is outdated: Expected {riot_version}, found: {curr_version}");
                }
            }
        },
        GenArgs::Wiki { function } => match function {
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
            Wiki::Runes => todo!(),
            Wiki::RunesLinks => runes::links().await,
            Wiki::RunesDownload => runes::download().await,
            Wiki::RunesParse => runes::parse(),
        }
        .map_err(|e| format!("[wiki] Error: {e:?}"))?,
    }

    Ok(())
}
