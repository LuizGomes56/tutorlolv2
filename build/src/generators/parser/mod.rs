use {
    crate::{
        MayFail,
        generators::{
            GeneratorExt, VERSION,
            utils::{SaveTo, Tag},
        },
        scripts::{
            batch::FmtArgs,
            utils::{StaticVar, is_zero, static_vars, variable},
        },
    },
    heck::{
        ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase,
        ToSnakeCase, ToTitleCase, ToTrainCase, ToUpperCamelCase,
    },
    rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    serde::{Deserialize, Serialize, de::DeserializeOwned},
    serde_json::{Value, json},
    std::{
        collections::{BTreeMap, BTreeSet, HashMap},
        ops::{Index, IndexMut},
        path::Path,
    },
    tutorlolv2_types::DamageIndex,
    tutorlolv2_wiki::{
        JsonRead, JsonWrite, champions::WikiChampion, items::item_parser::WikiItem, runes::WikiRune,
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

#[allow(dead_code)]
pub trait Parser<T, U>
where
    Self: Sized + Sync,
    T: Clone + DeserializeOwned + MapValueExt + Send + Sync + 'static,
    U: TryFrom<T, Error = Box<dyn core::error::Error + Send + Sync>> + Serialize,
{
    const TAG: Tag;
    const FN: fn(&str) -> Option<fn(T) -> MayFail<Box<dyn GeneratorExt<U>>>>;

    fn new() -> MayFail<Self>;
    fn map(&self) -> &BTreeMap<String, T>;
    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool>;

    fn tag(&self) -> Tag {
        Self::TAG
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

        let arguments = self
            .map()
            .iter()
            .map(|(key, value)| {
                let name = &value.name();

                let mut aliases = get_aliases(key, name);

                if let Some(extra) = &extras
                    && let Some(extra_aliases) = extra.get(key)
                {
                    extra_aliases.iter().cloned().for_each(|a| aliases.push(a));
                }

                let alias = BTreeSet::from_iter(aliases)
                    .into_iter()
                    .map(|v| format!("{v:?}"))
                    .collect::<Vec<_>>()
                    .join(" | ");

                format!("{alias} => {enum_name}::{key}")
            })
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "pub static {utag}_NAME_TO_ID: phf::Map<&str, {enum_name}> = phf::phf_map!({arguments});",
            utag = Self::TAG.singular().to_uppercase(),
        )
    }

    fn id_enum(&self) -> String {
        format!(
            "#[derive(
                Clone, Copy, Debug, Decode, Deserialize, Eq, Encode,
                Hash, Ord, PartialEq, PartialOrd, Serialize
            )]
            #[repr({repr})]
            pub enum {enum_name} {{{variants}}}

            impl {enum_name} {{
                pub const VARIANTS: usize = {len};
                pub const fn debug(&self) -> &'static str {{
                    match self {{{debug_arms}}}
                }}
                {riot_id_conv}
            }}",
            enum_name = Self::TAG.enum_name(),
            repr = Self::TAG.repr(),
            variants = self.keys().collect::<Vec<_>>().join(","),
            len = self.map().len(),
            debug_arms = self
                .keys()
                .map(|name| format!("Self::{name} => {name:?},"))
                .collect::<String>(),
            riot_id_conv = if !matches!(Self::TAG, Tag::Champions) {
                format!(
                    "pub const fn from_riot_id(id: u32) -> Option<Self> {{
                        match id {{ {match_arms} _ => None }}
                    }}",
                    match_arms = self
                        .map()
                        .iter()
                        .map(|(key, value)| {
                            format!("{riot_id} => Some(Self::{key}),", riot_id = value.riot_id())
                        })
                        .collect::<String>()
                )
            } else {
                String::new()
            }
        )
    }

    fn generator(id: &str, variant: &str) -> String {
        let folder = Self::TAG.plural();
        let mut default = false;
        let mut generator = tutorlolv2_wiki::read_to_string(format!(
            "build/src/generators/impls/{folder}/{file_name}.rs",
            file_name = id.to_snake_case()
        ))
        .unwrap_or_else(|_| {
            default = true;
            "impl Generator {}".into()
        });

        if let Some(pos) = generator.find("impl") {
            generator.drain(..pos);
        }

        generator.insert_str(
            0,
            &format!(
                "#[fmt({fmt})]",
                fmt = json!(FmtArgs {
                    target: "generator".into(),
                    variant: variant.into(),
                    meta: (),
                    default
                })
            ),
        );
        generator
    }

    fn static_vars<'a, const N: usize>(array: [StaticVar<'a>; N]) -> HashMap<&'a str, String> {
        static_vars(Self::TAG, array)
    }

    fn data_variable(&self) -> String {
        let vtype = Self::TAG.singular().to_pascal_case();
        let var = format!("{}S_DATA", vtype.to_uppercase());
        let mut data = variable(Self::TAG, &var, &format!("&{vtype}"));

        for id in self.keys() {
            let upper_id = id.to_shouty_snake_case();
            let lower_id = id.to_snake_case();
            data.push_str(&format!("&{lower_id}::{upper_id},"));
        }

        data.push_str("];");
        data
    }

    fn keys(&self) -> impl Iterator<Item = &str> + Send + Sync {
        self.map().keys().map(String::as_str)
    }

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
            let target = parent.join(Self::TAG.plural()).with_extension("json");
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
                tutorlolv2_wiki::write(&path, result)
            }
            false => Ok(tutorlolv2_wiki::remove_file(&path)),
        }
    }

    fn create_all(&self) -> MayFail {
        let keys = self.map().keys().map(String::as_str).collect::<Vec<_>>();
        let tag = Self::TAG;

        let dir_loc = SaveTo::GeneratorDir(tag).path();
        let dir = Path::new(&dir_loc);

        let decl = dir.join("mod").with_extension("rs");
        let module = format_args!("decl_{}", tag.plural());

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
