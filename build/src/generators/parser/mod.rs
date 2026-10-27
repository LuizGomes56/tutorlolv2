use {
    crate::{
        MayFail,
        generators::{GeneratorExt, VERSION, utils::SaveTo},
        scripts::utils::is_zero,
    },
    heck::{
        ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase,
        ToSnakeCase, ToTitleCase, ToTrainCase, ToUpperCamelCase,
    },
    rayon::iter::{IntoParallelRefIterator, ParallelIterator},
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    serde_json::Value,
    std::{
        collections::{BTreeMap, BTreeSet},
        ops::{Index, IndexMut},
        path::Path,
    },
    tutorlolv2_types::{DamageIndex, Tag},
    tutorlolv2_wiki::{
        JsonWrite, champions::WikiChampion, items::item_parser::WikiItem, runes::WikiRune,
    },
};

pub mod champions;
pub mod items;
pub mod model;
pub mod runes;

pub const ZERO: &str = "zero";

pub trait MapValueExt {
    fn riot_id(&self) -> u32;
    fn name(&self) -> &str;
}

impl MapValueExt for WikiChampion {
    fn riot_id(&self) -> u32 {
        panic!("Champions can't have riot_id fields")
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl MapValueExt for WikiItem {
    fn riot_id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl MapValueExt for WikiRune {
    fn riot_id(&self) -> u32 {
        self.riot_id as _
    }

    fn name(&self) -> &str {
        &self.name
    }
}

type GeneratorFnSig<T, U> = fn(T) -> MayFail<Box<dyn GeneratorExt<U>>>;

#[allow(dead_code)]
pub trait Parser<T, U>
where
    Self: Sized + Sync,
    T: Clone + DeserializeOwned + MapValueExt + Send + Sync + 'static,
    U: TryFrom<T, Error = Box<dyn core::error::Error + Send + Sync>> + Serialize,
{
    const TAG: Tag;
    const FN: fn(&str) -> Option<GeneratorFnSig<T, U>>;

    fn from_root(root: &Path) -> MayFail<Self>;

    fn new() -> MayFail<Self> {
        Self::from_root(&crate::project_root())
    }

    fn map(&self) -> &BTreeMap<String, T>;
    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool>;

    /// Attach catalog-wide metadata after the entity's custom generator has run.
    fn prepare(&self, _id: &str, _value: &mut U) {}

    fn tag(&self) -> Tag {
        Self::TAG
    }

    fn len(&self) -> usize {
        self.map().len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn phf(&self, extras: Option<BTreeMap<String, Vec<String>>>) -> String {
        let enum_name = Self::TAG.enum_name();

        fn get_aliases<'a>(id: &'a str, name: &'a str) -> Vec<String> {
            let get = |s: &str| {
                [
                    s.to_string(),
                    s.to_lowercase(),
                    s.to_uppercase(),
                    s.to_pascal_case(),
                    s.to_kebab_case(),
                    s.to_shouty_kebab_case(),
                    s.to_shouty_snake_case(),
                    s.to_lower_camel_case(),
                    s.to_upper_camel_case(),
                    s.to_title_case(),
                    s.to_train_case(),
                ]
            };

            [get(id), get(name)].concat()
        }

        let aliases_by_key = self
            .map()
            .iter()
            .map(|(key, value)| {
                let mut aliases = get_aliases(key, value.name());

                if let Some(extra) = &extras
                    && let Some(extra_aliases) = extra.get(key)
                {
                    extra_aliases.iter().cloned().for_each(|a| aliases.push(a));
                }

                (key.as_str(), value, BTreeSet::from_iter(aliases))
            })
            .collect::<Vec<_>>();

        // Items sharing a display name, like `JungleQuestReward1209`, also share the aliases
        // derived from it. Each alias goes to the variant with the lowest riot id, which is
        // the base game item, so `FromStr` resolves the same way on every build.
        let mut owners = BTreeMap::<&str, (&str, u32)>::new();

        for (key, value, aliases) in &aliases_by_key {
            let rank = match Self::TAG {
                Tag::Champions => 0,
                _ => value.riot_id(),
            };

            for alias in aliases {
                owners
                    .entry(alias)
                    .and_modify(|owner| {
                        if rank < owner.1 {
                            *owner = (key, rank);
                        }
                    })
                    .or_insert((key, rank));
            }
        }

        let arguments = aliases_by_key
            .iter()
            .filter_map(|(key, _, aliases)| {
                let alias = aliases
                    .iter()
                    .filter(|alias| owners[alias.as_str()].0 == *key)
                    .map(|v| format!("{v:?}"))
                    .collect::<Vec<_>>();

                (!alias.is_empty()).then(|| format!("{} => {enum_name}::{key}", alias.join(" | ")))
            })
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "pub static {utag}_NAME_TO_ID: phf::Map<&str, {enum_name}> = phf::phf_map!({arguments});",
            utag = Self::TAG.enum_name().trim_end_matches("Id").to_uppercase(),
        )
    }

    fn id_enum(&self) -> String {
        let enum_name = Self::TAG.enum_name();

        let riot_id_conv = if !matches!(Self::TAG, Tag::Champions) {
            // Sorted by riot id so `from_riot_id` is a binary search over a table instead
            // of a jump table with one arm per variant. A stable sort plus `dedup_by_key`
            // keeps the first variant for a repeated riot id, as the old `match` did.
            let mut riot_ids = self
                .map()
                .iter()
                .map(|(key, value)| (value.riot_id(), key.as_str()))
                .collect::<Vec<_>>();

            riot_ids.sort_by_key(|(riot_id, _)| *riot_id);
            riot_ids.dedup_by_key(|(riot_id, _)| *riot_id);

            format!(
                "impl {enum_name} {{
                    const SORTED_RIOT_IDS: [u32; {len}] = [{ids}];
                    const SORTED_RIOT_ID_VARIANTS: [Self; {len}] = [{variants}];

                    pub const fn from_riot_id(id: u32) -> Option<Self> {{
                        let (mut lo, mut hi) = (0, Self::SORTED_RIOT_IDS.len());
                        while lo < hi {{
                            let mid = lo + (hi - lo) / 2;
                            let key = Self::SORTED_RIOT_IDS[mid];
                            if key == id {{
                                return Some(Self::SORTED_RIOT_ID_VARIANTS[mid]);
                            }}
                            if key < id {{ lo = mid + 1 }} else {{ hi = mid }}
                        }}
                        None
                    }}
                }}",
                len = riot_ids.len(),
                ids = riot_ids
                    .iter()
                    .map(|(riot_id, _)| riot_id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                variants = riot_ids
                    .iter()
                    .map(|(_, key)| format!("Self::{key}"))
                    .collect::<Vec<_>>()
                    .join(","),
            )
        } else {
            String::new()
        };

        format!(
            "#[derive(
                Clone,
                Copy,
                Debug,
                Eq,
                Hash,
                Ord,
                PartialEq,
                PartialOrd,
                strum::EnumCount,
                strum::VariantArray,
                strum::VariantNames,
                strum::FromRepr
            )]
            #[repr({repr})]
            pub enum {enum_name} {{{variants}}}
            {riot_id_conv}",
            repr = Self::TAG.repr(),
            variants = self.keys().collect::<Vec<_>>().join(","),
        )
    }

    fn data_variable(&self) -> String {
        let vtype = Self::TAG.enum_name().trim_end_matches("Id");
        let var = format!("{}S_DATA", vtype.to_uppercase());

        let enum_name = Self::TAG.enum_name();
        let folder = Self::TAG.as_ref();

        let mut data = format!(
            "pub static {var}: [&crate::libgen::globals::{vtype}; <{enum_name} as strum::EnumCount>::COUNT] = ["
        );

        for id in self.keys() {
            let upper_id = id.to_shouty_snake_case();
            let lower_id = id.to_snake_case();

            data.push_str(&format!(
                "&crate::libgen::{folder}::{lower_id}::{upper_id},"
            ));
        }

        data.push_str("];");
        data
    }

    fn keys(&self) -> impl Iterator<Item = &str> + Send + Sync {
        self.map().keys().map(String::as_str)
    }

    fn run_fn(&self, id: &str) -> MayFail<U> {
        let mut value = self
            .map()
            .get(id)
            .map(|data| {
                let data = data.clone();

                match Self::FN(id) {
                    Some(f) => f(data)?.call(),
                    None => U::try_from(data),
                }
            })
            .ok_or_else(|| format!("[WikiFactory::run] {id} not found"))??;

        self.prepare(id, &mut value);

        Ok(value)
    }

    fn run_all(&self) -> MayFail {
        self.run_all_in(&crate::project_root())
    }

    fn run_all_in(&self, root: &Path) -> MayFail {
        self.map()
            .keys()
            .collect::<Vec<_>>()
            .par_iter()
            .map(|id| {
                let data = self.run_fn(id)?;
                Ok(((*id).clone(), self.export(id, &data, root)?))
            })
            .collect::<MayFail<BTreeMap<_, _>>>()?
            .write_file(
                root.join("internal")
                    .join(Self::TAG.as_ref())
                    .with_extension("json"),
            )
    }

    fn export(&self, id: &str, data: &U, root: &Path) -> MayFail<Value> {
        #[derive(Serialize)]
        struct TaskResult<'a, D> {
            #[serde(flatten)]
            data: D,
            version: &'a str,
            stable: bool,
        }

        let generator_path = root
            .join("build/src/generators/impls")
            .join(Self::TAG.as_ref())
            .join(id.to_snake_case())
            .with_extension("rs");

        let stable = match std::fs::read_to_string(&generator_path) {
            Ok(source) => !source.contains("#[warn(unstable_features)]"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => false,
            Err(e) => {
                return Err(format!("Reading generator status {generator_path:?}: {e}").into());
            }
        };

        let value = serde_json::to_value(TaskResult {
            data,
            version: VERSION,
            stable,
        })?;

        value.write_file(
            root.join("internal")
                .join(Self::TAG.as_ref())
                .join(id)
                .with_extension("json"),
        )?;

        Ok(value)
    }

    fn run(&self, id: &str) -> MayFail {
        let data = self
            .run_fn(id)
            .map_err(|e| format!("Generating {} {id}: {e}", Self::TAG.as_ref()))?;

        self.export(id, &data, &crate::project_root())?;

        Ok(())
    }

    fn is_generator(id: &str) -> bool {
        tutorlolv2_wiki::read_to_string(SaveTo::GeneratorRaw(Self::TAG, id).path()).is_ok()
    }

    fn is_stable(id: &str) -> bool {
        if let Ok(data) =
            tutorlolv2_wiki::read_to_string(SaveTo::GeneratorRaw(Self::TAG, id).path())
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
        let length = self.len();

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
                tutorlolv2_wiki::write(&path, result)
            }
            false => {
                tutorlolv2_wiki::remove_file(&path);
                Ok(())
            }
        }
    }

    fn create_all(&self) -> MayFail {
        let keys = self.map().keys().map(String::as_str).collect::<Vec<_>>();
        let tag = Self::TAG;

        let dir_loc = SaveTo::GeneratorDir(tag).path();
        let dir = Path::new(&dir_loc);

        let decl = dir.join("mod").with_extension("rs");
        let module = format_args!("decl_{}", tag.as_ref());

        keys.par_iter().try_for_each(|key| self.create(key))?;

        let modules = keys
            .iter()
            .copied()
            .filter(|key| {
                let loc = SaveTo::GeneratorRaw(Self::TAG, key).path();
                Path::new(&loc).exists()
            })
            .collect::<Vec<_>>()
            .join(",\n\t");

        let decl_content = format!(
            "use crate::generators::imports::{module}::*;\ncrate::{module}!(\n\t{modules}\n);",
        );

        tutorlolv2_wiki::write(&decl, decl_content)?;

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
    if let Some(dtype) = ["physical", "magic", "true"]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DamageRange {
    pub min_dmg: String,
    pub max_dmg: String,
}

impl DamageRange {
    pub fn deals_damage(&self) -> [bool; 2] {
        [!is_zero(&self.min_dmg), !is_zero(&self.max_dmg)]
    }
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
