//! Assets embedded into the binary. Paths mirror the web app's `public/` layout,
//! so `svgs/sidebar/Homepage.svg` here is `/svgs/sidebar/Homepage.svg` there. `other/` holds
//! the position icons from `tutorlolv2/img/other`.
use gpui::{AssetSource, Result, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "svgs/**/*.svg"]
#[include = "other/*.svg"]
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
