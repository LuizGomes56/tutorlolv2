//! Assets embedded into the binary. Paths mirror the web app's `public/` layout,
//! so `svgs/sidebar/Homepage.svg` here is `/svgs/sidebar/Homepage.svg` there. `other/` and
//! `stats/` come from `tutorlolv2/img`: the position icons, the basic attack and tower images
//! (the only non-AVIF rasters there) and the stat icons.
//!
//! gpui rasterizes an SVG at its intrinsic size before scaling it. `stats/crit_chance.svg` has
//! `width="100%"` over an 8144x8175 view box, which made a ~266 MB bitmap that failed to
//! render, so this copy sets `width="64" height="64"` instead.
use gpui::{AssetSource, Result, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "svgs/**/*.svg"]
#[include = "other/*.svg"]
#[include = "other/*.png"]
#[include = "other/*.webp"]
#[include = "stats/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|file| file.data))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|asset| asset.starts_with(path))
            .map(|asset| SharedString::from(asset.into_owned()))
            .collect())
    }
}
