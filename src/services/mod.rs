pub(crate) mod calculator;
mod helpers;
pub(crate) mod realtime;
mod riot_formulas;

use helpers::*;

pub enum CalculationError {
    CurrentPlayerNotFound,
    ChampionNameNotFound(String),
    ChampionCacheNotFound(String),
    ItemCacheNotFound(usize),
}
