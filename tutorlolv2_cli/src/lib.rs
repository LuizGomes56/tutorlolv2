use clap::{Parser, Subcommand};
use std::{process::Command, str::FromStr};
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
}

impl FromStr for RunTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "items" | "i" => Ok(Self::Factory(ItemFactory::run_all)),
            "champions" | "c" => Ok(Self::Factory(ChampionFactory::run_all)),
            _ => {
                if let Ok(champion_id) = ChampionId::from_str(s) {
                    return Ok(Self::Champion(champion_id));
                }

                if let Ok(item) = ItemId::from_str(s) {
                    return Ok(Self::Item(item));
                }

                Err(format!(
                    "Value {s:?} can't be converted to ChampionId or ItemId"
                ))
            }
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum GenArgs {
    #[command(alias = "c")]
    Create {
        champ: ChampionId,
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
        GenArgs::Create { champ } => {
            ChampionFactory::create(champ.debug())?;
        }
        GenArgs::Run { target } => match target {
            RunTarget::Champion(champ) => {
                ChampionFactory::run(champ.debug())?;
            }
            RunTarget::Item(item) => {
                ItemFactory::run(item.debug(), item.to_riot_id())?;
            }
            RunTarget::Factory(f) => f()?,
        },
        GenArgs::Progress => ChampionFactory::progress(),
        GenArgs::Update => {
            update::setup_project_folders()?;
            ChampionFactory::create_all()?;
            ChampionFactory::run_all()?;
            ItemFactory::run_all()?;
            Command::new("./build.bat").spawn()?.wait()?;
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
    }

    Ok(())
}
