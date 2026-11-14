//! Champion, item, rune and position images: the web app's `components/image.rs`.
//!
//! Until the WebP encoder exists, raster images come from `img/raw` (PNG/JPG),
//! read from disk. Set `TUTORLOLV2_RAW_IMG` to use another folder.
use gpui::{ImageSource, Img, IntoElement, div, img, prelude::*};
use math::{AbilityId, AbilityName, CastId, ChampionId, ItemId, Position, RuneId};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use strum::VariantArray;

fn raw_img_root() -> &'static Path {
    static ROOT: OnceLock<PathBuf> = OnceLock::new();

    ROOT.get_or_init(|| {
        std::env::var_os("TUTORLOLV2_RAW_IMG")
            .map(PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../img/raw"))
    })
}

fn raw_img(relative: String) -> ImageSource {
    ImageSource::from(Arc::<Path>::from(raw_img_root().join(relative)))
}

/// An image that renders as empty space when the file is missing.
pub fn image(source: ImageSource) -> Img {
    img(source).with_fallback(|| div().into_any_element())
}

/// Champions, items and runes: anything with a portrait, name and documentation.
pub trait Entry: CastId + VariantArray + PartialEq {
    fn image_source(&self) -> ImageSource;
}

impl Entry for ChampionId {
    fn image_source(&self) -> ImageSource {
        raw_img(format!("champions/{self:?}.png"))
    }
}

impl Entry for ItemId {
    fn image_source(&self) -> ImageSource {
        raw_img(format!("items/{}.png", self.riot_id()))
    }
}

impl Entry for RuneId {
    fn image_source(&self) -> ImageSource {
        raw_img(format!("runes/{}.png", self.riot_id()))
    }
}

/// The champion's splash art, cropped to the center (`ImageType::Centered`).
pub fn centered_image(champion: ChampionId) -> Img {
    image(raw_img(format!("centered/{champion:?}_0.jpg")))
}

/// An ability icon (`ImageType::Ability`). Mega Gnar's abilities use their own icons.
pub fn ability_image(champion: ChampionId, ability: AbilityId) -> Img {
    let suffix = match ability.ability_name() {
        AbilityName::Mega => "Mega",
        _ => "",
    };

    image(raw_img(format!(
        "abilities/{champion:?}{}{suffix}.png",
        ability.as_char()
    )))
}

/// An embedded image (`assets/`), such as `stats/armor.svg`.
pub fn asset_image(path: &'static str) -> Img {
    image(path.into())
}

/// Embedded position icon (`assets/other`).
pub fn position_image(position: Position) -> Img {
    let path = match position {
        Position::Top => "other/Top.svg",
        Position::Jungle => "other/Jungle.svg",
        Position::Middle => "other/Middle.svg",
        Position::Bottom => "other/Bottom.svg",
        Position::Support => "other/Support.svg",
    };

    image(path.into())
}
