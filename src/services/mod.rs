pub mod calculator;
mod helpers;
pub mod realtime;
mod riot_formulas;

use helpers::*;
use riot_formulas::*;

#[derive(Debug)]
pub enum CalculationError {
    CurrentPlayerNotFound,
    ChampionNameNotFound,
    ChampionCacheNotFound,
}

impl CalculationError {
    #[inline(always)]
    pub const fn as_str(self) -> &'static str {
        match self {
            CalculationError::CurrentPlayerNotFound => "Current player not found in allPlayers",
            CalculationError::ChampionNameNotFound => {
                "Could not convert champion name to its corresponding id"
            }
            CalculationError::ChampionCacheNotFound => "Current champion cache not found",
        }
    }
}
