use clap::{Parser, Subcommand};
use std::str::FromStr;
use tutorlolv2_dev::{
    MayFail,
    gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory},
    update,
};
use tutorlolv2_gen::{ChampionId, ItemId};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub args: GenArgs,
}

#[derive(Clone, Copy, Debug)]
pub enum RunTarget {
    Champion(ChampionId),
    Item(ItemId),
    Factory(fn() -> MayFail),
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
            _ => Err(format!(
                "Value {s:?} can't be converted to ChampionId or ItemId"
            )),
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
            _ => Err(format!("Value {s:?} can't be converted to ChampionId")),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum GenArgs {
    #[command(alias = "c")]
    Create {
        creator: GenCreator,
    },
    #[command(alias = "r")]
    Run {
        target: RunTarget,
    },
    #[command(alias = "p")]
    Progress,
    #[command(alias = "u")]
    Update,
    Html,
    #[command(alias = "s")]
    Setup {
        setup: Setup,
    },
    #[command(alias = "b")]
    Build,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Setup {
    #[clap(alias = "i")]
    Items,
    #[clap(alias = "p")]
    Prettify,
    #[clap(alias = "f")]
    Folders,
}

pub fn run() -> MayFail {
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
            RunTarget::Factory(f) => f()?,
            RunTarget::All => {
                ChampionFactory::run_all()?;
                ItemFactory::run_all()?;
            }
        },
        GenArgs::Progress => ChampionFactory::progress(),
        GenArgs::Update => {
            update::setup_project_folders()?;
            ChampionFactory::create_all()?;
            ChampionFactory::run_all()?;
            ItemFactory::run_all()?;
            std::env::set_current_dir("./tutorlolv2_build")?;
            tutorlolv2_build::run()?;
            std::env::set_current_dir("../")?;
            tutorlolv2_html::run();
        }
        GenArgs::Html => tutorlolv2_html::run(),
        GenArgs::Setup { setup } => match setup {
            Setup::Items => {
                update::setup_internal_items()?;
                update::prettify_internal_items()?;
            }
            Setup::Prettify => update::prettify_internal_items()?,
            Setup::Folders => update::setup_project_folders()?,
        },
        GenArgs::Build => {
            std::env::set_current_dir("./tutorlolv2_build")?;
            tutorlolv2_build::run()?
        }
    }

    Ok(())
}
