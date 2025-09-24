pub mod calculator;
pub mod helpers;
pub mod realtime;
pub mod riot_formulas;

use helpers::*;
use riot_formulas::*;
use std::{error::Error, fmt::Display};
use tutorlolv2_gen::{ChampionId, ItemId, RuneId};

#[derive(Debug)]
pub enum CalculationError {
    CurrentPlayerNotFound,
    ChampionNameNotFound,
    ChampionCacheNotFound,
}

impl Error for CalculationError {}

impl Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CalculationError::CurrentPlayerNotFound => "Current player not found in allPlayers",
                CalculationError::ChampionNameNotFound => {
                    "Could not convert champion name to its corresponding id"
                }
                CalculationError::ChampionCacheNotFound => "Current champion cache not found",
            }
        )
    }
}

pub trait Transmutable {
    fn transmute(repr: u16) -> Self;
}

macro_rules! impl_transmutable {
    ($ty:ty, $cast:ty) => {
        impl Transmutable for $ty {
            #[inline(always)]
            fn transmute(repr: u16) -> Self {
                unsafe { std::mem::transmute(repr as $cast) }
            }
        }
    };
}

impl_transmutable!(ItemId, u16);
impl_transmutable!(ChampionId, u8);
impl_transmutable!(RuneId, u8);
