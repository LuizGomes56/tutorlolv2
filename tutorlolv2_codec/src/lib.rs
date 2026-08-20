//! Build and render compact formula bytecode.
//!
//! This crate deliberately knows nothing about `ChampionId`, `ItemId`, `RuneId` or `CastId`.
//! Integration with those API types belongs in the consumer (`tutorlolv2` or a sibling crate).

pub mod build;
mod common;
pub mod render;

pub use build::{BuilderStats, CtxResolver, FormulaDbBuilder, FormulaSource};
pub use common::{DamageSlot, EntityKind, Error, MAGIC, VERSION};
pub use render::{Bracket, Class, FormulaDb, Highlighter};
