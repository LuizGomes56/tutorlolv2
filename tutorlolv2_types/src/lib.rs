#![no_std]
#![allow(clippy::missing_transmute_annotations)]

extern crate alloc;

/// Shared binary serialization settings; callers may add decoding size limits.
pub const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

mod ability;
mod attack_speed_ratio;
mod ctx;
mod damage;
mod entity;
mod game;
mod metadata;
mod stats;

#[cfg(feature = "yew")]
pub mod yew;

pub use {
    ability::*, attack_speed_ratio::*, ctx::*, damage::*, entity::*, game::*, metadata::*, stats::*,
};

use bincode::{Decode, Encode};
use core::{fmt::Display, str::FromStr};
use serde::{Deserialize, Serialize};
use strum::EnumCount;

// Keep the existing string error type when deriving parsers.
type ParseError = &'static str;
