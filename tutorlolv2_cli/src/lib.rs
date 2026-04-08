use clap::{Parser, Subcommand};
use std::str::FromStr;
use tutorlolv2_dev::gen_factories::{fac_champions::ChampionFactory, fac_items::ItemFactory};
use tutorlolv2_gen::{ChampionId, ItemId};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub args: GenArgs,
}

#[derive(Debug, Clone, Copy)]
pub enum RunTarget {
    Champion(ChampionId),
    Item(ItemId),
}

impl FromStr for RunTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(champ) = s.parse::<ChampionId>() {
            return Ok(Self::Champion(champ));
        }

        if let Ok(item) = s.parse::<ItemId>().or(ItemId::VALUES
            .iter()
            .find(|item_id| format!("{item_id:?}") == s)
            .copied()
            .ok_or("Unable to parse ItemId based on variant name"))
        {
            return Ok(Self::Item(item));
        }

        Err(format!(
            "Value provided can't be parsed to ChampionId or ItemId: {s}"
        ))
    }
}

#[derive(Subcommand, Debug)]
pub enum GenArgs {
    #[command(alias = "c")]
    Create { champ: ChampionId },
    #[command(alias = "r")]
    Run { target: RunTarget },
    #[command(alias = "p")]
    Progress,
}

pub fn run() {
    let Cli { args } = Cli::parse();

    std::env::set_current_dir("../").unwrap();

    match args {
        GenArgs::Create { champ } => {
            ChampionFactory::create(&format!("{champ:?}")).unwrap();
        }
        GenArgs::Run { target } => match target {
            RunTarget::Champion(champ) => {
                ChampionFactory::run(&format!("{champ:?}")).unwrap();
            }
            RunTarget::Item(item) => {
                ItemFactory::run(&format!("{item:?}"), item.to_riot_id()).unwrap();
            }
        },
        GenArgs::Progress => {
            ChampionFactory::progress();
            ItemFactory::progress();
        }
    }
}
