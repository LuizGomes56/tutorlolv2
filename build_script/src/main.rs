mod scripts;
use scripts::*;
use shared_types::AbilityLike;

const ONHIT_EFFECT: &'static str = r#"<pre><span class="control">intrinsic</span> <span class="constant">ONHIT_EFFECT</span> = {
    <span class="variable">name</span>: <span class="string">"Onhit Effect"</span>,
    <span class="variable">damage_type</span>: <span class="type">DamageType</span>::<span class="constant">Mixed</span>,
    <span class="variable">damage</span>: ($($<span class="variable">dmg</span>:<span class="variable">expr</span>),*) => {
        <span class="variable">onhit_effects</span>.<span class="variable">minimum_damage</span>
            += <span class="constant">BASIC_ATTACK</span> + $($<span class="variable">dmg</span>),*;
        <span class="variable">onhit_effects</span>.<span class="variable">maximum_damage</span>
            += <span class="constant">CRITICAL_STRIKE</span> + $($<span class="variable">dmg</span>),*;
    },
};
</pre>"#;

#[tokio::main]
async fn main() {
    let mut mega_block = String::with_capacity(1 << 24);
    let champion_languages_raw_json =
        std::fs::read_to_string(cwd!("internal/champion_languages.json")).unwrap();
    let champion_languages_json =
        serde_json::from_str::<HashMap<String, Vec<String>>>(&champion_languages_raw_json).unwrap();

    let champions_handle = tokio::task::spawn_blocking(move || export_champions());
    let items_handle = tokio::task::spawn_blocking(move || export_items());
    let runes_handle = tokio::task::spawn_blocking(move || export_runes());

    let champions = champions_handle.await.unwrap();
    let items = items_handle.await.unwrap();
    let runes = runes_handle.await.unwrap();

    let mut internal_champions_content = String::with_capacity(1 << 16);
    let mut internal_champions = format!(
        "pub static INTERNAL_CHAMPIONS: [&'static CachedChampion; {}] = [",
        champions.len()
    );
    let mut champion_id_to_name = format!(
        "pub static CHAMPION_ID_TO_NAME: [&'static str; {}] = [",
        champions.len()
    );
    let mut champion_formulas = format!(
        "pub static CHAMPION_FORMULAS: [(usize, usize); {}] = [",
        champions.len()
    );
    let mut champion_generator = format!(
        "pub static CHAMPION_GENERATOR: [(usize, usize); {}] = [",
        champions.len()
    );
    let mut champion_abilities = format!(
        "pub static CHAMPION_ABILITIES: [&'static [(AbilityLike, (usize, usize))]; {}] = [",
        champions.len(),
    );
    let mut recommended_items = format!(
        "pub static RECOMMENDED_ITEMS: [[&'static [ItemId]; 5]; {}] = [",
        champions.len(),
    );
    let mut internal_items = format!(
        "pub static INTERNAL_ITEMS: [&'static CachedItem; {}] = [",
        items.len(),
    );
    let mut internal_items_content = String::new();
    let mut internal_simulated_items =
        String::from("pub static SIMULATED_ITEMS: phf::OrderedSet<u32> = phf::phf_ordered_set!(");
    let mut internal_damaging_items =
        String::from("pub static DAMAGING_ITEMS: phf::Set<u32> = phf::phf_set!(");
    let mut item_id_to_name = format!(
        "pub static ITEM_ID_TO_NAME: [&'static str; {}] = [",
        items.len(),
    );
    let mut item_formulas = format!(
        "pub static ITEM_FORMULAS: [(usize, usize); {}] = [",
        items.len(),
    );
    let mut item_descriptions = format!(
        "pub static ITEM_DESCRIPTIONS: [ItemDescription; {}] = [",
        items.len(),
    );
    let mut rune_id_to_name = format!(
        "pub static RUNE_ID_TO_NAME: [&'static str; {}] = [",
        runes.len(),
    );
    let mut rune_formulas = format!(
        "pub static RUNE_FORMULAS: [(usize, usize); {}] = [",
        runes.len(),
    );
    let mut internal_runes = format!(
        "pub static INTERNAL_RUNES: [&'static CachedRune; {}] = [",
        runes.len(),
    );
    let mut internal_runes_content = String::new();

    let mut current_offset = 0usize;

    macro_rules! record_offsets {
        ($value:expr) => {{
            let (start, end) = record_offsets!(@record $value);
            format!("({},{})", start, end)
        }};
        ($field:ident, $value:expr) => {{
            let (start, end) = record_offsets!(@record $value);
            $field.push_str(&format!("({},{}),", start, end));
        }};
        (@record $value:expr) => {{
            let start = current_offset;
            mega_block.push_str(&$value);
            let end = current_offset + $value.len();
            current_offset = end;
            (start, end)
        }}
    }

    let transform_and_join = |v: &[u32]| -> String {
        v.iter()
            .filter_map(|value| {
                items
                    .iter()
                    .find(|item| item.0 == *value)
                    .and_then(|matched_value| {
                        Some(format!(
                            "ItemId::{}",
                            remove_special_chars(&matched_value.1.item_name)
                        ))
                    })
            })
            .collect::<Vec<String>>()
            .join(",")
    };

    let meta_items_map = init_map!(file BTreeMap<String, Positions>, "internal/meta_items.json");
    let champion_meta_items = format!(
        "pub static META_ITEMS:[CachedMetaItem;{}]=[{}];",
        champions.len(),
        champions
            .iter()
            .map(|(champion_id, _)| {
                let positions = meta_items_map.get(champion_id).unwrap();
                let top = transform_and_join(&positions.top);
                let mid = transform_and_join(&positions.mid);
                let jungle = transform_and_join(&positions.jungle);
                let adc = transform_and_join(&positions.adc);
                let support = transform_and_join(&positions.support);
                recommended_items.push_str(&format!(
                    "[&[{}],&[{}],&[{}],&[{}],&[{}]],",
                    top, mid, jungle, adc, support
                ));
                format!(
                    "CachedMetaItem{{top:&[{}],mid:&[{}],jungle:&[{}],adc:&[{}],support:&[{}]}}",
                    top, mid, jungle, adc, support
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    );
    let champion_name_to_id_phf = format!(
        "pub static CHAMPION_NAME_TO_ID:phf::OrderedMap<&'static str,ChampionId>=phf::phf_ordered_map!{{{}}};",
        champions
            .iter()
            .map(|(key, _)| {
                champion_languages_json
                    .get(key)
                    .unwrap()
                    .iter()
                    .map(|champion_name_alias| {
                        format!(
                            "\"{}\"=>ChampionId::{}",
                            champion_name_alias,
                            remove_special_chars(key)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            })
            .collect::<Vec<String>>()
            .join(","),
    );
    let champion_id_enum = format!(
        "#[derive(Debug,PartialEq,Ord,Eq,PartialOrd,Copy,Clone,Decode,Encode)]
        #[repr(u8)]
        pub enum ChampionId {{{}}}
        impl ChampionId {{
            pub const fn as_str(&self)->&'static str{{match self{{{}}}}}
        }}",
        champions
            .iter()
            .map(|(key, _)| { remove_special_chars(key) })
            .collect::<Vec<String>>()
            .join(","),
        champions
            .iter()
            .map(|(key, _)| {
                format!(
                    "Self::{} => \"{}\"",
                    remove_special_chars(key),
                    remove_special_chars(key)
                )
            })
            .collect::<Vec<String>>()
            .join(",")
    );
    for (champion_id, champion_detail) in champions.into_iter() {
        internal_champions.push_str(&format!("&{},", champion_id.to_uppercase()));
        champion_id_to_name.push_str(&format!("\"{}\",", champion_detail.champion_name));
        record_offsets!(champion_formulas, champion_detail.champion_formula);
        record_offsets!(champion_generator, champion_detail.generator);

        champion_abilities.push_str("&[");
        for (ability_name, ability_formula) in &champion_detail.highlighted_abilities {
            let (start, end) = record_offsets!(@record ability_formula);
            champion_abilities.push_str(&format!(
                "({},({},{})),",
                AbilityLike::from_str(&ability_name),
                start,
                end
            ));
        }
        champion_abilities.push_str("],");
        internal_champions_content.push_str(&champion_detail.constdecl);
    }

    recommended_items.push_str("];");
    internal_champions.push_str("];");
    champion_id_to_name.push_str("];");
    champion_formulas.push_str("];");
    champion_generator.push_str("];");
    champion_abilities.push_str("];");

    let moved_champion_id_enum = champion_id_enum.clone();
    tokio::task::spawn_blocking(move || {
        fs::write(cwd!("internal_comptime/src/data/champions.rs"), {
            let mut s = String::with_capacity(
                USE_SUPER.len()
                    + internal_champions_content.len()
                    + internal_champions.len()
                    + moved_champion_id_enum.len()
                    + BASIC_ATTACK.len()
                    + CRITICAL_STRIKE.len()
                    + champion_name_to_id_phf.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(&moved_champion_id_enum);
            s.push_str(&internal_champions_content);
            s.push_str(&champion_name_to_id_phf);
            s.push_str(&internal_champions);
            s.push_str(&BASIC_ATTACK);
            s.push_str(&CRITICAL_STRIKE);
            s
        })
    });

    let mut internal_damaging_items_len = 0usize;
    let mut internal_simulated_items_len = 0usize;
    let item_id_to_u32 = format!(
        "pub static ITEM_ID_TO_U32:[u32;{}]=[{}];",
        items.len(),
        items
            .iter()
            .map(|(key, _)| *key)
            .collect::<Vec<u32>>()
            .join(","),
    );
    let item_id_enum = format!(
        "#[derive(Debug,Copy,Clone,Ord,Eq,PartialOrd,PartialEq,Decode,Encode)]
        #[repr(u16)]
        pub enum ItemId {{{}}}
        impl ItemId {{
            pub fn to_u32(&self)->u32{{unsafe{{*ITEM_ID_TO_U32.get_unchecked(*self as usize)}}}}
            pub const fn from_u32(id:u32)->Self{{match id {{{}}}}}  
        }}",
        items
            .iter()
            .map(|(_, value)| remove_special_chars(&value.item_name))
            .collect::<Vec<String>>()
            .join(","),
        items
            .iter()
            .map(|(item_id, value)| format!(
                "{}=>Self::{}",
                item_id,
                remove_special_chars(&value.item_name)
            ))
            .chain(std::iter::once("_=>Self::YourCut".to_string(),))
            .collect::<Vec<String>>()
            .join(","),
    );
    for (item_id, item_detail) in items.into_iter() {
        internal_items_content.push_str(&item_detail.constdecl);
        internal_items.push_str(&format!(
            "&{}_{},",
            item_detail.item_name.to_screaming_snake_case(),
            item_id
        ));
        if item_detail.is_simulated {
            internal_simulated_items.push_str(&format!("{}u32,", item_id));
            internal_simulated_items_len += 1;
        }
        if item_detail.is_damaging {
            internal_damaging_items.push_str(&format!("{}u32,", item_id));
            internal_damaging_items_len += 1;
        }
        record_offsets!(item_formulas, item_detail.item_formula);
        item_descriptions.push_str(&format!("{},", item_detail.description));
        item_id_to_name.push_str(&format!("\"{}\",", item_detail.item_name));
    }

    internal_items.push_str("];");
    internal_simulated_items.push_str(");");
    internal_damaging_items.push_str(");");
    item_id_to_name.push_str("];");
    item_formulas.push_str("];");
    item_descriptions.push_str("];");

    let moved_item_id_to_u32 = item_id_to_u32.clone();
    let moved_item_id_enum = item_id_enum.clone();
    tokio::task::spawn_blocking(move || {
        fs::write(cwd!("internal_comptime/src/data/items.rs"), {
            const META_ITEM_STRUCT: &'static str = "pub struct CachedMetaItem {
                pub jungle:&'static[ItemId],
                pub top:&'static[ItemId],
                pub mid:&'static[ItemId],
                pub adc:&'static[ItemId],
                pub support:&'static[ItemId],
            }";
            let mut s = String::with_capacity(
                internal_items.len()
                    + moved_item_id_enum.len()
                    + moved_item_id_to_u32.len()
                    + internal_items_content.len()
                    + internal_simulated_items.len()
                    + internal_damaging_items.len()
                    + champion_meta_items.len()
                    + META_ITEM_STRUCT.len()
                    + USE_SUPER.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(META_ITEM_STRUCT);
            s.push_str(&internal_items);
            s.push_str(&champion_meta_items);
            s.push_str(&moved_item_id_enum);
            s.push_str(&moved_item_id_to_u32);
            s.push_str(&internal_items_content);
            s.push_str(&internal_simulated_items);
            s.push_str(&internal_damaging_items);
            s.push_str(&format!(
                "pub const SIZE_SIMULATED_ITEMS:usize={};
                pub const SIZE_DAMAGING_ITEMS:usize={};",
                internal_simulated_items_len, internal_damaging_items_len
            ));
            s
        })
    });

    let internal_damaging_runes = format!(
        "pub static DAMAGING_RUNES:phf::Set<u32>=phf::phf_set!({});",
        runes
            .iter()
            .map(|(rune_id, _)| format!("{}u32", rune_id))
            .collect::<Vec<String>>()
            .join(",")
    );

    let internal_damaging_runes_len = runes.len();
    let rune_id_to_u32 = format!(
        "pub static RUNE_ID_TO_U32:[u32;{}]=[{}];",
        runes.len(),
        runes
            .iter()
            .map(|(key, _)| *key)
            .collect::<Vec<u32>>()
            .join(","),
    );
    let rune_id_enum = format!(
        "#[derive(Debug,Copy,Clone,Ord,Eq,PartialOrd,PartialEq,Decode,Encode)]
        #[repr(u8)]
        pub enum RuneId {{{}}}
        impl RuneId {{
            pub fn to_u32(&self)->u32{{unsafe{{*RUNE_ID_TO_U32.get_unchecked(*self as usize)}}}}
            pub const fn from_u32(id:u32)->Self{{match id{{{}}}}}
        }}",
        runes
            .iter()
            .map(|(_, value)| remove_special_chars(&value.rune_name))
            .collect::<Vec<String>>()
            .join(","),
        runes
            .iter()
            .map(|(rune_id, value)| format!(
                "{}=>Self::{}",
                rune_id,
                remove_special_chars(&value.rune_name)
            ))
            .chain(std::iter::once(format!(
                "_=>Self::{}",
                remove_special_chars(&runes.iter().next().unwrap().1.rune_name)
            )))
            .collect::<Vec<String>>()
            .join(","),
    );
    for (rune_id, rune_detail) in runes.into_iter() {
        internal_runes.push_str(&format!(
            "&{}_{},",
            rune_detail.rune_name.to_screaming_snake_case(),
            rune_id
        ));
        internal_runes_content.push_str(&rune_detail.constdecl);
        rune_id_to_name.push_str(&format!("\"{}\",", rune_detail.rune_name));
        record_offsets!(rune_formulas, rune_detail.rune_formula);
    }

    internal_runes.push_str("];");
    rune_id_to_name.push_str("];");
    rune_formulas.push_str("];");

    let moved_rune_id_to_u32 = rune_id_to_u32.clone();
    let moved_rune_id_enum = rune_id_enum.clone();
    tokio::task::spawn_blocking(move || {
        fs::write(cwd!("internal_comptime/src/data/runes.rs"), {
            let mut s = String::with_capacity(
                internal_runes.len()
                    + internal_runes_content.len()
                    + USE_SUPER.len()
                    + internal_damaging_runes.len()
                    + moved_rune_id_to_u32.len()
                    + moved_rune_id_enum.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(&moved_rune_id_to_u32);
            s.push_str(&moved_rune_id_enum);
            s.push_str(&internal_runes);
            s.push_str(&internal_runes_content);
            s.push_str(&internal_damaging_runes);
            s.push_str(&format!(
                "pub const SIZE_DAMAGING_RUNES:usize={};",
                internal_damaging_runes_len
            ));
            s
        })
    });

    let exported_content = [
        champion_id_enum,
        champion_id_to_name,
        champion_formulas,
        champion_generator,
        champion_abilities,
        recommended_items,
        item_id_to_name,
        item_formulas,
        item_descriptions,
        item_id_enum,
        item_id_to_u32,
        rune_id_enum,
        rune_id_to_u32,
        rune_id_to_name,
        rune_formulas,
    ]
    .concat();

    let mut bytes = Vec::from(b"use crate::*;");

    bytes.extend_from_slice(
        format!(
            "{}
            pub static BASIC_ATTACK_OFFSET:(usize,usize)={};
            pub static CRITICAL_STRIKE_OFFSET:(usize,usize)={};
            pub static ONHIT_EFFECT_OFFSET:(usize,usize)={};
            pub static UNCOMPRESSED_MEGA_BLOCK_SIZE:usize={};",
            exported_content,
            record_offsets!(highlight(&invoke_rustfmt(&BASIC_ATTACK, 60))),
            record_offsets!(highlight(&invoke_rustfmt(&CRITICAL_STRIKE, 60))),
            record_offsets!(ONHIT_EFFECT),
            current_offset,
        )
        .as_bytes(),
    );

    let _ = fs::write(cwd!("comptime_exports/mega_block.txt"), &mega_block);

    let mega_block_compressed = compress_bytes!(mega_block.as_bytes());

    bytes.extend_from_slice(
        format!(
            "pub static MEGA_BLOCK:[u8;{}]=[{}];",
            mega_block_compressed.len(),
            mega_block_compressed.join(",")
        )
        .as_bytes(),
    );

    let _ = fs::write(cwd!("comptime_exports/export_code.txt"), bytes);
}
