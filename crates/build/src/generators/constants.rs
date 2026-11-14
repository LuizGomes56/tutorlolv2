use crate::MayFail;
use heck::ToSnakeCase;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fmt::Write};
use types::MapMode;

/// Game value extracted from the wiki by a generator, emitted as a `pub const`
/// in the module of its champion, item or rune, so it follows the wiki between
/// patches instead of being hardcoded in the main crate
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Constant {
    Scalar(f64),
    /// One value for each rank of an ability
    Ranked(Vec<f64>),
}

impl Constant {
    fn ty(&self) -> String {
        match self {
            Self::Scalar(_) => "f32".into(),
            Self::Ranked(values) => format!("[f32; {}]", values.len()),
        }
    }

    fn literal(&self) -> String {
        match self {
            Self::Scalar(value) => format!("{:?}", *value as f32),
            Self::Ranked(values) => {
                format!("{:?}", values.iter().map(|&v| v as f32).collect::<Vec<_>>())
            }
        }
    }
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
pub struct Constants {
    values: BTreeMap<String, Constant>,
    /// Values on the modes that change a constant, for the constants that game
    /// modes may change. Each of them also gets a `const fn` of the map
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    maps: BTreeMap<String, Vec<(MapMode, Constant)>>,
}

impl Constants {
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Declares that game modes may change the constant `name`, with its value on
    /// each mode in `values`. The constant gets a `const fn` of the map even when
    /// no mode changes it, so the code that reads it doesn't depend on the wiki
    pub fn insert_map(&mut self, name: &str, values: Vec<(MapMode, Constant)>) -> MayFail {
        let Some(value) = self.values.get(name) else {
            return Err(format!("Constant {name} must be declared before its map values").into());
        };

        for (mode, other) in &values {
            let same_kind = match (value, other) {
                (Constant::Scalar(_), Constant::Scalar(_)) => true,
                (Constant::Ranked(a), Constant::Ranked(b)) => a.len() == b.len(),
                _ => false,
            };

            if !same_kind {
                return Err(format!("Constant {name} has another shape on {mode:?}").into());
            }

            if *mode == MapMode::Default {
                return Err(format!("Constant {name} can't be changed on the default map").into());
            }
        }

        if self.maps.insert(name.to_string(), values).is_some() {
            return Err(format!("Map values of {name} were declared twice").into());
        }

        Ok(())
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

        if self.values.insert(name.to_string(), value).is_some() {
            return Err(format!("Constant {name} was declared twice").into());
        }

        Ok(())
    }

    /// Rust declarations of every constant, as [`f32`] like the rest of the engine
    pub fn bake(&self) -> String {
        let mut result = String::new();

        for (name, value) in &self.values {
            let _ = writeln!(
                result,
                "pub const {name}: {} = {};",
                value.ty(),
                value.literal()
            );

            let Some(maps) = self.maps.get(name) else {
                continue;
            };

            // Nearly every game is on Summoner's Rift, so the other arms are cold
            let arms = maps
                .iter()
                .filter(|(_, other)| other != value)
                .map(|(mode, value)| {
                    format!(
                        "types::MapMode::{mode:?} => {{ core::hint::cold_path(); {} }}",
                        value.literal()
                    )
                })
                .collect::<String>();

            let fn_name = name.to_snake_case();
            let ty = value.ty();

            // No mode changes it this patch, but the code that reads it keeps its function
            let _ = match arms.is_empty() {
                true => writeln!(
                    result,
                    "/// [`{name}`] on `map`, which no mode changes\npub const fn {fn_name}(_map: types::MapMode) -> {ty} {{ {name} }}"
                ),
                false => writeln!(
                    result,
                    "/// [`{name}`] on `map`\npub const fn {fn_name}(map: types::MapMode) -> {ty} {{ match map {{ {arms} _ => {name} }} }}"
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

    /// Declares the values of the constant `name` on the modes that change it, see
    /// [`Constants::insert_map`]
    fn map_constant<T: Into<Constant>>(
        &mut self,
        name: &str,
        values: impl IntoIterator<Item = (MapMode, T)>,
    ) -> MayFail<&mut Self> {
        let values = values
            .into_iter()
            .map(|(mode, v)| (mode, v.into()))
            .collect();

        self.constants_mut().insert_map(name, values)?;

        Ok(self)
    }
}
