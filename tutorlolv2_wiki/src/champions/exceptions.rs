//! Wiki texts that hold values the generators need, but that are not parsed
//! into abilities. They are downloaded with every patch, together with the rest
//! of the champion, and saved in [`WikiChampion::exceptions`] by their name
//!
//! [`WikiChampion::exceptions`]: crate::champions::WikiChampion::exceptions

use crate::{HTTP_CLIENT, MayFail, champions::cache, parser::get_cells};
use scraper::Html;
use std::{collections::BTreeMap, path::PathBuf};

pub struct Exception {
    pub champion_id: &'static str,
    /// Name that generators use to read this text
    pub name: &'static str,
    /// Wiki page, relative to `https://wiki.leagueoflegends.com/en-us/`
    pub page: &'static str,
    /// Parameter of the template data table that holds the text
    pub cell: &'static str,
}

pub const EXCEPTIONS: &[Exception] = &[
    // The stat increases of Mega Gnar are not in any template of Gnar
    Exception {
        champion_id: "Gnar",
        name: "Mega Gnar",
        page: "Template:Data_Gnar/Mega_Gnar",
        cell: "description",
    },
    // The passive of Steadfast Presence has no leveling, so it is not kept as an effect
    Exception {
        champion_id: "Poppy",
        name: "Stubborn to a Fault",
        page: "Template:Data_Poppy/Steadfast_Presence",
        cell: "description",
    },
];

impl Exception {
    fn html_path(&self) -> PathBuf {
        cache()
            .join(self.champion_id)
            .join("exceptions")
            .join(self.name)
            .with_extension("html")
    }

    /// Path of the exceptions of a champion, by their name
    pub fn json_path(champion_id: &str) -> PathBuf {
        cache()
            .join(champion_id)
            .join("exceptions")
            .with_extension("json")
    }

    fn text(&self) -> MayFail<String> {
        let html = Html::parse_document(&crate::read_to_string(self.html_path())?);

        get_cells(&html)?.remove(self.cell).ok_or_else(|| {
            format!(
                "Exception {:?} of {} has no {:?} cell in {}",
                self.name, self.champion_id, self.cell, self.page
            )
            .into()
        })
    }
}

pub async fn download() -> MayFail {
    println!("[fn] champions::exceptions::download");

    for exception in EXCEPTIONS {
        HTTP_CLIENT
            .fetch(exception.html_path(), exception.page)
            .await?;
    }

    Ok(())
}

pub fn parse() -> MayFail {
    println!("[fn] champions::exceptions::parse");

    let mut by_champion = BTreeMap::<&str, BTreeMap<&str, String>>::new();

    for exception in EXCEPTIONS {
        by_champion
            .entry(exception.champion_id)
            .or_default()
            .insert(exception.name, exception.text()?);
    }

    for (champion_id, exceptions) in by_champion {
        crate::write(
            Exception::json_path(champion_id),
            serde_json::to_string_pretty(&exceptions)?,
        )?;
    }

    Ok(())
}

/// Exceptions of a champion, empty if it has none
pub fn read(champion_id: &str) -> MayFail<BTreeMap<String, String>> {
    match crate::read(Exception::json_path(champion_id)) {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
        Err(_) if !EXCEPTIONS.iter().any(|e| e.champion_id == champion_id) => Ok(BTreeMap::new()),
        Err(e) => Err(e),
    }
}
