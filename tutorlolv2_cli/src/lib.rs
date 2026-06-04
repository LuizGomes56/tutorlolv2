use clap::{Parser, Subcommand, ValueEnum};
use std::{convert::Infallible, str::FromStr, sync::LazyLock};
use tutorlolv2_dev::{
    ENV_CONFIG, HTTP_CLIENT, MayFail,
    gen_factories::{
        Parser as _, wiki_champions::ChampionParser, wiki_items::ItemParser, wiki_runes::RuneParser,
    },
};
use tutorlolv2_gen::{ChampionId, ItemId, RuneId};
use tutorlolv2_wiki::{champions, items, runes};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub args: AppArgs,
}

#[derive(Clone, Debug)]
pub enum EntityTarget {
    Champion(ChampionId),
    Item(ItemId),
    Rune(RuneId),
    Champions,
    Items,
    Runes,
    All,
    Unknown(String),
}

impl FromStr for EntityTarget {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "all" | "a" => Ok(Self::All),
            "c" | "champion" => Ok(Self::Champions),
            "i" | "item" => Ok(Self::Items),
            "r" | "rune" => Ok(Self::Runes),
            s if let Ok(champion_id) = ChampionId::from_str(s) => Ok(Self::Champion(champion_id)),
            s if let Ok(item_id) = ItemId::from_str(s) => Ok(Self::Item(item_id)),
            s if let Ok(rune_id) = RuneId::from_str(s) => Ok(Self::Rune(rune_id)),
            s => Ok(Self::Unknown(s.to_string())),
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
pub enum AppArgs {
    #[command(alias = "c")]
    Create { creator: EntityTarget },
    #[command(alias = "r")]
    Run { target: EntityTarget },
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
}

static IPARSER: LazyLock<ItemParser> = LazyLock::new(|| ItemParser::new().unwrap());
static CPARSER: LazyLock<ChampionParser> = LazyLock::new(|| ChampionParser::new().unwrap());
static RPARSER: LazyLock<RuneParser> = LazyLock::new(|| RuneParser::new().unwrap());

pub async fn run() -> MayFail {
    let Cli { args } = Cli::parse();

    dotenvy::dotenv().expect(".env file not found");
    std::env::set_current_dir("../")?;

    match args {
        AppArgs::Create { creator } => match creator {
            EntityTarget::All => {
                CPARSER.create_all()?;
                IPARSER.create_all()?;
                RPARSER.create_all()?;
            }
            EntityTarget::Champion(v) => CPARSER.create(v.debug())?,
            EntityTarget::Champions => CPARSER.create_all()?,
            EntityTarget::Item(v) => IPARSER.create(v.debug())?,
            EntityTarget::Items => IPARSER.create_all()?,
            EntityTarget::Rune(v) => RPARSER.create(v.debug())?,
            EntityTarget::Runes => RPARSER.create_all()?,
            EntityTarget::Unknown(s) => panic!("Can't create generator for unknown string {s}"),
        },
        AppArgs::Run { target } => match target {
            EntityTarget::All => {
                CPARSER.run_all();
                IPARSER.run_all();
                RPARSER.run_all();
            }
            EntityTarget::Champion(v) => CPARSER.run(v.debug())?,
            EntityTarget::Champions => CPARSER.run_all(),
            EntityTarget::Item(v) => IPARSER.run(v.debug())?,
            EntityTarget::Items => IPARSER.run_all(),
            EntityTarget::Rune(v) => RPARSER.run(v.debug())?,
            EntityTarget::Runes => RPARSER.run_all(),
            EntityTarget::Unknown(s) => RPARSER.run(&s)?,
        },
        AppArgs::Progress => {
            println!("Champions:");
            CPARSER.progress();
            println!("Items:");
            IPARSER.progress();
            println!("Runes:");
            RPARSER.progress();
        }
        AppArgs::Update => {
            tutorlolv2_wiki::run().await?;
            HTTP_CLIENT.update_riot_cache().await?;

            // CPARSER.create_all()?;
            // IPARSER.create_all()?;
            // RPARSER.create_all()?;

            CPARSER.run_all();
            IPARSER.run_all();
            RPARSER.run_all();

            HTTP_CLIENT.download_arts_img().await?;
            HTTP_CLIENT.download_items_img().await?;
            HTTP_CLIENT.download_runes_img().await?;
            HTTP_CLIENT.download_general_img().await?;

            // let _ = HTTP_CLIENT.call_scraper().await;
            // let _ = HTTP_CLIENT.combo_scraper().await;

            tutorlolv2_build::run()?;
        }
        AppArgs::Html => tutorlolv2_html::run(),
        AppArgs::Setup { setup } => match setup {
            Setup::Items => {
                /* update::setup_runes_json()? */
                todo!()
            }
            Setup::Prettify => {
                /* update::prettify_internal_items()? */
                todo!()
            }
        },
        AppArgs::Build => tutorlolv2_build::run()?,
        AppArgs::Fetch { function } => match function {
            Fetch::Images => {
                HTTP_CLIENT.download_arts_img().await?;
                HTTP_CLIENT.download_items_img().await?;
                HTTP_CLIENT.download_runes_img().await?;
                HTTP_CLIENT.download_general_img().await?;
            }
            Fetch::Cache => HTTP_CLIENT.update_riot_cache().await?,
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
