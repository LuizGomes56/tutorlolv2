use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::{Arguments, Debug},
    ops::Range,
};
use tutorlolv2_dev::{
    decl_champions::Champion, decl_items::Item, decl_runes::Rune, gen_factories::ZERO,
};
use tutorlolv2_fmt::{pascal_case, to_ssnake};

pub trait MapValueExt {
    fn riot_id(&self) -> u32;
    fn name(&self) -> &str;
}

impl MapValueExt for Champion {
    fn riot_id(&self) -> u32 {
        panic!("Champions can't have riot_id fields")
    }

    fn name(&self) -> &str {
        &self.data.name
    }
}

impl MapValueExt for Item {
    fn riot_id(&self) -> u32 {
        self.build.riot_id
    }

    fn name(&self) -> &str {
        &self.data.name
    }
}

impl MapValueExt for Rune {
    fn riot_id(&self) -> u32 {
        self.build.riot_id
    }

    fn name(&self) -> &str {
        &self.data.name
    }
}

#[derive(Debug)]
pub enum Tag {
    Champion,
    Item,
    Rune,
}

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        match self {
            Tag::Item => "Item",
            Tag::Rune => "Rune",
            Tag::Champion => "Champion",
        }
    }
}

pub fn get_name_phf<T: MapValueExt>(
    data: &BTreeMap<String, T>,
    tag: Tag,
    extras: Option<BTreeMap<String, Vec<String>>>,
) -> String {
    let arguments = data
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

            format!("{alias} => {tag:?}Id::{key}")
        })
        .collect::<String>();

    format!(
        "pub static {utag}_NAME_TO_ID: phf::Map<&str, {tag:?}Id> = phf::phf_map!({arguments});",
        utag = tag.as_ref().to_uppercase(),
    )
}

pub fn get_id_enum<T: MapValueExt>(data: &BTreeMap<String, T>, tag: Tag) -> String {
    format!(
        "
        #[derive(
            Clone, Copy, Debug, Decode, Deserialize, Eq, Encode,
            Hash, Ord, PartialEq, PartialOrd, Serialize
        )]
        #[repr(u8)]
        pub enum {tag:?}Id {{{variants}}}

        impl {tag:?}Id {{
            pub const VARIANTS: usize = {len};
            pub const fn debug(&self) -> &'static str {{
                match self {{{debug_arms}}}
            }}
            {riot_id_conv}
        }}
        ",
        variants = data
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(","),
        len = data.len(),
        debug_arms = data
            .keys()
            .map(|name| format!("Self::{name} => {name:?},"))
            .collect::<String>(),
        riot_id_conv = if !matches!(tag, Tag::Champion) {
            format!(
                "pub const fn from_riot_id(id: u32) -> Option<Self> {{
                    match id {{ {match_arms} _ => None }}
                }}",
                match_arms = data
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

pub fn get_const_eval(data: &BTreeMap<&String, Batch>, tag: Tag) -> String {
    format!(
        "
        pub const fn {ltag}_const_eval(
            ctx: &Ctx,
            {ltag}_id: {tag:?}Id,
            attack_type: AttackType
        ) -> [f32; 2] {{
            match {ltag}_id {{ {eval} _ => [0.0, 0.0] }}
        }}
        ",
        ltag = tag.as_ref().to_lowercase(),
        eval = data
            .values()
            .map(|batch| batch.eval.as_str())
            .collect::<Vec<&str>>()
            .concat()
    )
}

pub fn get_generator(tag: Tag, id: &str, variant: &str) -> String {
    let folder = tag.as_ref().to_lowercase();
    let mut generator = tutorlolv2_dev::read_to_string(format!(
        "tutorlolv2_dev/src/generators/gen_{folder}s/{file_name}.rs",
        file_name = id.to_lowercase()
    ))
    .unwrap_or("impl Generator {}".into());

    if let Some(pos) = generator.find("impl") {
        generator.drain(..pos);
    }

    generator.insert_str(0, &format!("#[fmt(generator, {variant})]"));
    generator
}

pub fn get_eval(
    tag: Tag,
    id: &str,
    deals_damage: &[bool; 4],
    melee: &[String],
    ranged: &[String],
) -> String {
    let get_arms = |range: Range<_>, array: &[String]| {
        deals_damage[range]
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let f = match *v {
                    true => &array[i],
                    false => ZERO,
                };
                format!("{f}(&ctx)")
            })
            .collect::<Vec<_>>()
            .join(",")
    };

    format!(
        "
            {tag:?}Id::{id} => {{
                match attack_type {{
                    Melee => [{melee_arms}],
                    Ranged => [{ranged_arms}]
                }}
            }},
            ",
        melee_arms = get_arms(0..2, melee),
        ranged_arms = get_arms(2..4, ranged),
    )
}

pub struct StaticVar {
    pub attribute: &'static str,
    pub name: &'static str,
    pub vtype: &'static str,
}

pub struct FmtArgs {
    pub variable: String,
    pub output: Vec<Range<usize>>,
}

pub fn get_static_vars<const N: usize, T>(
    tag: Tag,
    data: &BTreeMap<String, T>,
    array: [StaticVar; N],
) -> (String, HashMap<&'static str, FmtArgs>) {
    let make = |name: &str, vtype| {
        format!(
            "pub static {var}: [{vtype}; {tag:?}Id::VARIANTS] = [",
            var = name.to_uppercase()
        )
    };

    let mut cache = make(&format!("{tag:?}_CACHE"), tag.as_ref());

    for id in data.keys() {
        let upper_id = id.to_uppercase();
        cache.push_str(&format!("&{upper_id},"));
    }

    cache.push_str("];");

    let fmt_args = array
        .into_iter()
        .map(|static_var| {
            let StaticVar {
                attribute,
                name,
                vtype,
            } = static_var;
            let variable = make(name, vtype);
            (
                attribute,
                FmtArgs {
                    variable,
                    output: Vec::new(),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    (cache, fmt_args)
}

pub fn closures(
    functions: &[[String; 2]; 2],
    melee: &[String],
    ranged: &[String],
    variant: &str,
) -> String {
    functions
        .iter()
        .enumerate()
        .map(|(i, function)| {
            function
                .iter()
                .enumerate()
                .map(|(j, function)| {
                    let body = &match i {
                        0 => melee,
                        1 => ranged,
                        _ => unreachable!(),
                    }[j];

                    let default = body == ZERO || body == "0";

                    let formula = simplify(body);
                    let closure = if default {
                        format_args!("pub const fn {function}(ctx: &Ctx) -> f32 {{{formula}}}")
                    } else {
                        format_args!("")
                    };

                    format!(
                        r#"
                        {closure}

                        #[fmt(
                            target = closure,
                            variant = {variant},
                            replace = ["ctx." => ""],
                            default = {default}
                        )]
                        fn {function}() {{{formula}}}
                        "#
                    )
                })
                .collect::<String>()
        })
        .collect::<String>()
}

pub fn simplify(formula: &str) -> String {
    symb_anafis::simplify(&formula.replace("ctx.", "ctx_"), &[], None)
        .map(|r| r.replace("ctx_", "ctx."))
        .unwrap_or(formula.to_string())
}

pub fn slice_repr<T: Debug>(slice: &[T]) -> String {
    slice
        .iter()
        .map(|ident| format!("&{ident:#?}"))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn get_aliases<'a>(id: &'a str, name: &'a str) -> Vec<String> {
    let get = |s: &str| {
        [
            s.to_string(),
            s.to_lowercase(),
            s.to_uppercase(),
            pascal_case(s),
            pascal_case(s).to_lowercase(),
            pascal_case(s).to_uppercase(),
            to_ssnake(s),
            to_ssnake(s).to_lowercase(),
            to_ssnake(s).to_uppercase(),
        ]
    };

    [get(id), get(name)].concat()
}

pub struct Batch {
    pub eval: String,
    pub fmt: String,
}
