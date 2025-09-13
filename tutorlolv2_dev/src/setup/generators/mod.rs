pub mod champions;
pub mod extractors;
mod generator_runner;
pub mod items;
pub mod runes;

use crate::{
    model::{champions::*, items::*},
    setup::{essentials::helpers::*, generators::generator_runner::try_run_generator},
};
use extractors::*;
