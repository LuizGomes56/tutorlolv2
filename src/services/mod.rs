pub mod calculator;
mod helpers;
pub mod realtime;
mod riot_formulas;

use helpers::*;
use riot_formulas::*;

pub enum CalculationError {
    CurrentPlayerNotFound,
    ChampionNameNotFound(String),
    ChampionCacheNotFound(String),
    ItemCacheNotFound(u32),
}
