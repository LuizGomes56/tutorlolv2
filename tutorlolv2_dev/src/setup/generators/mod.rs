pub mod champions;
pub mod decl_v2;
pub mod extractors;
mod generator_runner;
pub mod items;
pub mod runes;

use crate::{
    model::{champions::*, items::*},
    setup::{essentials::ext::*, generators::generator_runner::try_run_generator},
};
use extractors::*;

pub mod champion_v2;
