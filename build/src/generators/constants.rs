use crate::MayFail;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Write};

/// Game value extracted from the wiki by a generator, emitted as a `pub const`
/// in the module of its champion, item or rune, so it follows the wiki between
/// patches instead of being hardcoded in the main crate
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Constant {
    Scalar(f64),
    /// One value for each rank of an ability
    Ranked(Vec<f64>),
}

impl From<f64> for Constant {
    fn from(value: f64) -> Self {
        Self::Scalar(value)
    }
}

impl From<Vec<f64>> for Constant {
    fn from(value: Vec<f64>) -> Self {
        Self::Ranked(value)
    }
}

impl<const N: usize> From<[f64; N]> for Constant {
    fn from(value: [f64; N]) -> Self {
        Self::Ranked(value.to_vec())
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Constants(BTreeMap<String, Constant>);

impl Constants {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, name: &str, value: impl Into<Constant>) -> MayFail {
        let is_shouty_snake = name.chars().next().is_some_and(|c| c.is_ascii_uppercase())
            && name
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');

        if !is_shouty_snake {
            return Err(format!("Constant name {name:?} must be SHOUTY_SNAKE_CASE").into());
        }

        let value = value.into();

        if let Constant::Ranked(values) = &value
            && values.is_empty()
        {
            return Err(format!("Constant {name} has no values").into());
        }

        if self.0.insert(name.to_string(), value).is_some() {
            return Err(format!("Constant {name} was declared twice").into());
        }

        Ok(())
    }

    /// Rust declarations of every constant, as [`f32`] like the rest of the engine
    pub fn bake(&self) -> String {
        let mut result = String::new();

        for (name, value) in &self.0 {
            let _ = match value {
                Constant::Scalar(value) => {
                    writeln!(result, "pub const {name}: f32 = {:?};", *value as f32)
                }
                Constant::Ranked(values) => writeln!(
                    result,
                    "pub const {name}: [f32; {}] = {:?};",
                    values.len(),
                    values.iter().map(|&v| v as f32).collect::<Vec<_>>()
                ),
            };
        }

        result
    }
}

/// Entities whose generators can declare [`Constants`]
pub trait WithConstants {
    fn constants_mut(&mut self) -> &mut Constants;

    fn constant(&mut self, name: &str, value: impl Into<Constant>) -> MayFail<&mut Self> {
        self.constants_mut().insert(name, value)?;
        Ok(self)
    }
}
