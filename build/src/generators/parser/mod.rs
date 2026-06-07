use crate::{
    MayFail,
    generators::{
        GeneratorExt, VERSION,
        utils::{SaveTo, Tag},
    },
};
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use regex::Regex;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
    path::Path,
    sync::LazyLock,
};
use tutorlolv2_dev::{JsonRead, JsonWrite};
use tutorlolv2_fmt::rustfmt;
use tutorlolv2_types::{CtxVar, DamageIndex, DamageType};

pub mod champions;
pub mod items;
pub mod runes;

pub const ZERO: &str = "zero";

pub trait Parser<T, U>
where
    Self: Sized + Sync,
    T: Clone + DeserializeOwned + Send + Sync + 'static,
    U: TryFrom<T, Error = Box<dyn core::error::Error + Send + Sync>> + Serialize,
{
    const TAG: Tag;
    const FN: fn(&str) -> Option<fn(T) -> MayFail<Box<dyn GeneratorExt<U>>>>;

    fn new() -> MayFail<Self>;
    fn map(&self) -> &BTreeMap<String, T>;
    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool>;

    fn run_fn(&self, id: &str) -> MayFail<U> {
        self.map()
            .get(id)
            .map(|data| {
                let data = data.clone();

                match Self::FN(id) {
                    Some(f) => f(data)?.call(),
                    None => U::try_from(data),
                }
            })
            .ok_or_else(|| format!("[WikiFactory::run] {id} not found"))?
    }

    fn run_all(&self) {
        self.map().keys().par_bridge().for_each(|key| {
            let _ = self.run(key);
        });

        let dir = SaveTo::InternalDir(Self::TAG).path();
        let path = Path::new(&dir);

        if let Some(parent) = path.parent() {
            let target = parent.join(Self::TAG.to_string()).with_extension("json");
            let _ = Value::from_dir(dir).map(|r| r.into_file(target));
        }
    }

    fn run(&self, id: &str) -> MayFail {
        #[derive(Serialize)]
        struct TaskResult<'a, D> {
            #[serde(flatten)]
            pub data: D,
            pub version: &'a str,
            pub stable: bool,
        }

        match self.run_fn(id) {
            Ok(data) => TaskResult {
                data,
                version: VERSION,
                stable: Self::is_stable(id),
            }
            .into_file(SaveTo::InternalRaw(Self::TAG, id).path()),
            Err(e) => Ok(println!("Error generating {id:?}: {e:?}")),
        }
    }

    fn is_generator(id: &str) -> bool {
        tutorlolv2_dev::read_to_string(SaveTo::GeneratorRaw(Self::TAG, id).path()).is_ok()
    }

    fn is_stable(id: &str) -> bool {
        if let Ok(data) = tutorlolv2_dev::read_to_string(SaveTo::GeneratorRaw(Self::TAG, id).path())
            && !data.contains("#[warn(unstable_features)]")
        {
            return true;
        }

        false
    }

    fn progress(&self) {
        let mut stables = 0;
        let mut unstables = 0;

        for name in self.map().keys() {
            if !Self::is_generator(name) {
                continue;
            }

            if Self::is_stable(name) {
                stables += 1;
                continue;
            }

            unstables += 1;
        }

        let total = stables + unstables;
        let length = self.map().len();

        let print = |a, b, tag| {
            println!(
                "{a:>3} / {b:>3} {tag:<10} ({ratio:.1}%)",
                ratio = a as f32 / b as f32 * 100.0
            );
        };

        print(stables, total, "stable");
        print(unstables, total, "unstable");
        print(total, length, "generators");
    }

    fn create(&self, id: &str) -> MayFail {
        if !self.map().keys().any(|k| k == id) {
            return Err(format!("[WikiFactory::create] {id} not found").into());
        }

        if Self::is_stable(id) {
            println!("[stable] Skipping generator for {id:?}");
            return Ok(());
        }

        let mut result = format!(
            "use super::*;

            impl Generator for {id} {{
                #[warn(unstable_features)]
                fn generate(&mut self) -> MayFail {{ self"
        );

        let path = SaveTo::GeneratorRaw(Self::TAG, id).path();

        match self.create_methods(&mut result, id)? {
            true => {
                result.push_str(".end()}}");

                let formatted = rustfmt(&result, None);
                let content = match formatted.is_empty() {
                    true => result,
                    false => formatted,
                };

                tutorlolv2_dev::write(&path, content)
            }
            false => Ok(tutorlolv2_dev::remove_file(&path)),
        }
    }

    fn create_all(&self) -> MayFail {
        let keys = self.map().keys().map(String::as_str).collect::<Vec<_>>();
        let tag = Self::TAG;

        let dir_loc = SaveTo::GeneratorDir(tag).path();
        let dir = Path::new(&dir_loc);

        let decl = dir.join("mod").with_extension("rs");
        let module = format_args!("decl_{tag}");

        keys.par_iter().try_for_each(|key| self.create(key))?;

        let modules = keys
            .iter()
            .copied()
            .filter_map(|key| {
                let loc = SaveTo::GeneratorRaw(Self::TAG, key).path();
                Path::new(&loc).exists().then_some(key)
            })
            .collect::<Vec<_>>()
            .join(",\n\t");

        let decl_content = format!(
            "use crate::generators::imports::{module}::*;\ncrate::{module}!(\n\t{modules}\n);",
        );

        tutorlolv2_dev::write(&decl, decl_content)?;

        Ok(())
    }
}

pub fn likely_damages(text: &str) -> bool {
    let mut isad = false;

    for word in text.split(|c: char| !c.is_ascii_alphabetic()) {
        if word.is_empty() {
            continue;
        }

        if word.eq_ignore_ascii_case("damage") {
            return !isad;
        }

        isad = word.eq_ignore_ascii_case("attack");
    }

    false
}

pub fn infer_damage_type(result: &mut String, description: &str) {
    if let Some(dtype) = ["physical", "physical", "true"]
        .into_iter()
        .find(|d| description.to_lowercase().contains(d))
    {
        let alias = dtype
            .chars()
            .next()
            .map(|c| c.to_uppercase().collect::<String>() + &dtype[1..])
            .unwrap_or(dtype.to_string());

        result.push_str(&format!(".damage_type({alias})"));
    }
}

pub fn is_zero(value: &str) -> bool {
    value == ZERO || value == "0" || value == "0.0" || value == "(0.0)" || value == "(0)"
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DamageRange {
    pub min_dmg: String,
    pub max_dmg: String,
}

impl Index<DamageIndex> for DamageRange {
    type Output = String;

    fn index(&self, index: DamageIndex) -> &Self::Output {
        match index {
            DamageIndex::Min => &self.min_dmg,
            DamageIndex::Max => &self.max_dmg,
        }
    }
}

impl IndexMut<DamageIndex> for DamageRange {
    fn index_mut(&mut self, index: DamageIndex) -> &mut Self::Output {
        match index {
            DamageIndex::Min => &mut self.min_dmg,
            DamageIndex::Max => &mut self.max_dmg,
        }
    }
}

impl Default for DamageRange {
    fn default() -> Self {
        Self {
            min_dmg: ZERO.into(),
            max_dmg: ZERO.into(),
        }
    }
}

pub fn get_identifiers(damage: &str, damage_type: DamageType) -> impl Iterator<Item = CtxVar> + '_ {
    static RE_IDENTS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"ctx\.([a-z_][a-z0-9_]*)").unwrap());

    RE_IDENTS
        .captures_iter(&damage)
        .filter_map(|cap| tutorlolv2_fmt::pascal_case(&cap[1]).parse().ok())
        .chain(match damage_type {
            DamageType::Physical => Some(CtxVar::PhysicalMultiplier),
            DamageType::Magic => Some(CtxVar::MagicMultiplier),
            _ => None,
        })
}
