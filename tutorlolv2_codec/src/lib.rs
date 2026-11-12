//! Build and render compact formula bytecode.
//!
//! This crate deliberately knows nothing about `ChampionId`, `ItemId`, `RuneId` or `CastId`.
//! Integration with those API types belongs in the consumer (`tutorlolv2` or a sibling crate).

pub mod build;
mod common;
pub mod generator;
pub mod markup;
pub mod render;

pub use build::{BuilderStats, CtxResolver, FormulaDbBuilder, FormulaSource, MapResolver};
pub use common::{DamageSlot, EntityKind, Error, MAGIC, ParseError, VERSION};
pub use generator::{
    ClassOverride, GeneratorBuildOutput, GeneratorDb, GeneratorDbBuilder, GeneratorError,
    GeneratorStats,
};
pub use markup::{Html, Markup, Span, Spans};
pub use render::{Bracket, Class, FormulaDb, GenericHighlighter, Highlighter};
