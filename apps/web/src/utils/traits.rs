use crate::{components::image::ImageType, utils::random_u64};
use math::{CastId, ChampionId, ItemId, RuneId};
use std::fmt::Debug;
use strum::EnumCount;

pub trait EnumCast
where
    Self: CastId + TryFrom<u16> + PartialEq + Copy + EnumCount,
    ImageType: From<Self>,
{
    fn random() -> Self {
        let index = random_u64(0..Self::COUNT as _);

        unsafe { Self::try_from(index as u16).unwrap_unchecked() }
    }

    fn image_type(&self) -> ImageType {
        (*self).into()
    }
}

impl EnumCast for ChampionId {}
impl EnumCast for ItemId {}
impl EnumCast for RuneId {}

pub trait Print: Debug {
    fn log(&self) {
        web_sys::console::log_1(&format!("{self:#?}").into());
    }
}

impl<T: Debug> Print for T {}
