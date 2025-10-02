mod scripts;

use scripts::*;
use tutorlolv2_fmt::{encode_brotli_11, highlight_rust, invoke_rustfmt};
use tutorlolv2_types::AbilityLike;

const ONHIT_EFFECT: &str = r#"<pre><span class="control">intrinsic</span> <span class="constant">ONHIT_EFFECT</span> = {
    <span class="variable">name</span>: <span class="string">"Onhit Effect"</span>,
    <span class="variable">damage_type</span>: <span class="type">DamageType</span>::<span class="constant">Mixed</span>,
    <span class="variable">damage</span>: ($($<span class="variable">dmg</span>:<span class="variable">expr</span>),*) => {
        <span class="variable">onhit</span>.<span class="variable">minimum_damage</span>
            += <span class="constant">BASIC_ATTACK</span> + $($<span class="variable">dmg</span>),*;
        <span class="variable">onhit</span>.<span class="variable">maximum_damage</span>
            += <span class="constant">CRITICAL_STRIKE</span> + $($<span class="variable">dmg</span>),*;
    },
};
</pre>"#;

pub async fn run() {
    let mut mega_block = String::with_capacity(1 << 23);
    let champion_languages_raw_json =
        std::fs::read_to_string(cwd!("internal/champion_languages.json")).unwrap();
    let champion_languages_json =
        serde_json::from_str::<HashMap<String, Vec<String>>>(&champion_languages_raw_json).unwrap();

    let champions_handle = tokio::task::spawn_blocking(export_champions);
    let items_handle = tokio::task::spawn_blocking(export_items);
    let runes_handle = tokio::task::spawn_blocking(export_runes);

    let champions = champions_handle.await.unwrap();
    let items = items_handle.await.unwrap();
    let runes = runes_handle.await.unwrap();

    let mut internal_champions_content = String::with_capacity(1 << 16);
    let mut internal_champions = format!(
        "pub static INTERNAL_CHAMPIONS:[&CachedChampion;{}]=[",
        champions.len()
    );
    let mut champion_positions = format!(
        "pub static CHAMPION_POSITIONS:[&[Position];{}]=[",
        champions.len()
    );
    let mut champion_id_to_name = format!(
        "pub static CHAMPION_ID_TO_NAME:[&str;{}]=[",
        champions.len()
    );
    let mut champion_formulas = format!(
        "pub static CHAMPION_FORMULAS:[(u32,u32);{}]=[",
        champions.len()
    );
    let mut champion_generator = format!(
        "pub static CHAMPION_GENERATOR:[(u32,u32);{}]=[",
        champions.len()
    );
    let mut champion_abilities = format!(
        "pub static CHAMPION_ABILITIES:[&[(AbilityLike,(u32,u32))];{}]=[",
        champions.len(),
    );
    let mut recommended_items = format!(
        "pub static RECOMMENDED_ITEMS:[[&[ItemId];5];{}]=[",
        champions.len(),
    );
    let mut recommended_runes = format!(
        "pub static RECOMMENDED_RUNES:[[&[RuneId];5];{}]=[",
        champions.len(),
    );
    let mut internal_items = format!("pub static INTERNAL_ITEMS:[&CachedItem;{}]=[", items.len(),);
    let mut internal_items_content = String::new();
    let mut internal_simulated_items =
        String::from("pub static SIMULATED_ITEMS:phf::OrderedSet<u32>=phf::phf_ordered_set!(");
    let mut internal_simulated_items_enum = format!(
        "pub static SIMULATED_ITEMS_ENUM:[u32;{}]=[",
        items.iter().filter(|(_, v)| v.is_simulated).count()
    );
    let mut internal_damaging_items =
        String::from("pub static DAMAGING_ITEMS:phf::Set<u32>=phf::phf_set!(");
    let mut item_id_to_name = format!("pub static ITEM_ID_TO_NAME:[&str;{}]=[", items.len(),);
    let mut item_formulas = format!("pub static ITEM_FORMULAS:[(u32,u32);{}]=[", items.len(),);
    let mut rune_id_to_name = format!("pub static RUNE_ID_TO_NAME:[&str;{}]=[", runes.len(),);
    let mut rune_formulas = format!("pub static RUNE_FORMULAS:[(u32,u32);{}]=[", runes.len(),);
    let mut internal_runes = format!("pub static INTERNAL_RUNES:[&CachedRune;{}]=[", runes.len(),);
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

    let scraped_data_map =
        init_map!(file BTreeMap<String, Positions>, "internal/scraped_data.json");

    for champion_id in champions.keys() {
        let positions = scraped_data_map.get(champion_id).unwrap();
        let mut item_results = Vec::new();
        let mut rune_results = Vec::new();
        for (pos_items, pos_runes) in positions.make_iterable().iter() {
            let insert_array = |array: &[String], enum_kind: &str| {
                array
                    .iter()
                    .map(|element| format!("{}::{}", enum_kind, element).to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            };
            item_results.push(insert_array(pos_items, "ItemId"));
            rune_results.push(insert_array(pos_runes, "RuneId"));
        }
        let push_result = |array: &[String], target_string: &mut String| {
            target_string.push_str(&format!(
                "[{}],",
                array
                    .iter()
                    .map(|element| format!("&[{}]", element).to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ))
        };
        push_result(&item_results, &mut recommended_items);
        push_result(&rune_results, &mut recommended_runes);
    }

    let champion_name_to_id_phf = format!(
        "pub static CHAMPION_NAME_TO_ID:phf::Map<&'static str,ChampionId>=phf::phf_map!{{{}}};",
        champions
            .keys()
            .map(|key| {
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
        #[repr(u8)]pub enum ChampionId {{{}}}",
        champions
            .keys()
            .map(|key| remove_special_chars(key))
            .collect::<Vec<String>>()
            .join(","),
    );
    for (champion_id, champion_detail) in champions.into_iter() {
        internal_champions.push_str(&format!("&{},", champion_id.to_uppercase()));
        champion_id_to_name.push_str(&format!("\"{}\",", champion_detail.champion_name));
        champion_positions.push_str(&format!("&[{}],", champion_detail.positions));
        record_offsets!(champion_formulas, champion_detail.champion_formula);
        record_offsets!(champion_generator, champion_detail.generator);

        champion_abilities.push_str("&[");
        for (ability_name, ability_formula) in &champion_detail.highlighted_abilities {
            let (start, end) = record_offsets!(@record ability_formula);
            champion_abilities.push_str(&format!(
                "({},({},{})),",
                AbilityLike::from_str(ability_name),
                start,
                end
            ));
        }
        champion_abilities.push_str("],");
        internal_champions_content.push_str(&champion_detail.constdecl);
    }

    recommended_items.push_str("];");
    recommended_runes.push_str("];");
    champion_positions.push_str("];");
    internal_champions.push_str("];");
    champion_id_to_name.push_str("];");
    champion_formulas.push_str("];");
    champion_generator.push_str("];");
    champion_abilities.push_str("];");

    let moved_champion_id_enum = champion_id_enum.clone();
    tokio::task::spawn_blocking(move || {
        fs::write(cwd!("tutorlolv2_gen/src/data/champions.rs"), {
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
            s.push_str(BASIC_ATTACK);
            s.push_str(CRITICAL_STRIKE);
            s
        })
    });

    let item_id_to_riot_id = format!(
        "pub static ITEM_ID_TO_RIOT_ID:[u32;{}]=[{}];",
        items.len(),
        items
            .iter()
            .map(|(key, _)| *key)
            .collect::<Vec<u32>>()
            .join(","),
    );
    let item_id_enum = format!(
        "#[derive(Debug,Copy,Clone,Ord,Eq,PartialOrd,PartialEq,Decode,Encode)]#[repr(u16)]
        pub enum ItemId {{{}}}impl ItemId {{pub fn to_riot_id(&self)->u32
        {{unsafe{{*ITEM_ID_TO_RIOT_ID.get_unchecked(*self as usize)}}}}
        pub const fn from_riot_id(id:u32)->Self{{match id {{{}}}}}}}",
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
    for (index, (item_id, item_detail)) in items.into_iter().enumerate() {
        internal_items_content.push_str(&item_detail.constdecl);
        internal_items.push_str(&format!(
            "&{}_{},",
            item_detail.item_name.to_screaming_snake_case(),
            item_id
        ));
        if item_detail.is_simulated {
            internal_simulated_items.push_str(&format!("{}u32,", item_id));
            internal_simulated_items_enum.push_str(&format!("{},", index));
        }
        if item_detail.is_damaging {
            internal_damaging_items.push_str(&format!("{}u32,", item_id));
        }
        record_offsets!(item_formulas, item_detail.item_formula);
        item_id_to_name.push_str(&format!("\"{}\",", item_detail.item_name));
    }

    internal_items.push_str("];");
    internal_simulated_items.push_str(");");
    internal_simulated_items_enum.push_str("];");
    internal_damaging_items.push_str(");");
    item_id_to_name.push_str("];");
    item_formulas.push_str("];");

    let moved_item_id_to_riot_id = item_id_to_riot_id.clone();
    let moved_item_id_enum = item_id_enum.clone();
    tokio::task::spawn_blocking(move || {
        fs::write(cwd!("tutorlolv2_gen/src/data/items.rs"), {
            let mut s = String::with_capacity(
                internal_items.len()
                    + moved_item_id_enum.len()
                    + moved_item_id_to_riot_id.len()
                    + internal_items_content.len()
                    + internal_simulated_items.len()
                    + internal_simulated_items_enum.len()
                    + internal_damaging_items.len()
                    + USE_SUPER.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(&internal_items);
            s.push_str(&moved_item_id_enum);
            s.push_str(&moved_item_id_to_riot_id);
            s.push_str(&internal_items_content);
            s.push_str(&internal_simulated_items);
            s.push_str(&internal_simulated_items_enum);
            s.push_str(&internal_damaging_items);
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

    let rune_id_to_riot_id = format!(
        "pub static RUNE_ID_TO_RIOT_ID:[u32;{}]=[{}];",
        runes.len(),
        runes
            .iter()
            .map(|(key, _)| *key)
            .collect::<Vec<u32>>()
            .join(","),
    );
    let rune_id_enum = format!(
        "#[derive(Debug,Copy,Clone,Ord,Eq,PartialOrd,PartialEq,Decode,Encode)]#[repr(u8)]pub enum RuneId {{{}}}
        impl RuneId {{pub fn to_riot_id(&self)->u32{{unsafe{{*RUNE_ID_TO_RIOT_ID.get_unchecked(*self as usize)}}}}
        pub const fn from_riot_id(id:u32)->Self{{match id{{{}}}}}}}",
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
                remove_special_chars(&runes.first().unwrap().1.rune_name)
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

    let moved_rune_id_to_riot_id = rune_id_to_riot_id.clone();
    let moved_rune_id_enum = rune_id_enum.clone();
    tokio::task::spawn_blocking(move || {
        fs::write(cwd!("tutorlolv2_gen/src/data/runes.rs"), {
            let mut s = String::with_capacity(
                internal_runes.len()
                    + internal_runes_content.len()
                    + USE_SUPER.len()
                    + internal_damaging_runes.len()
                    + moved_rune_id_to_riot_id.len()
                    + moved_rune_id_enum.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(&moved_rune_id_to_riot_id);
            s.push_str(&moved_rune_id_enum);
            s.push_str(&internal_runes);
            s.push_str(&internal_runes_content);
            s.push_str(&internal_damaging_runes);
            s
        })
    });

    let exported_content = [
        champion_id_enum,
        champion_id_to_name,
        champion_positions,
        champion_formulas,
        champion_generator,
        champion_abilities,
        recommended_items,
        recommended_runes,
        item_id_to_name,
        item_formulas,
        item_id_enum,
        item_id_to_riot_id,
        rune_id_enum,
        rune_id_to_riot_id,
        rune_id_to_name,
        rune_formulas,
    ]
    .concat();

    let mut bytes = Vec::from(b"use crate::*;");

    bytes.extend_from_slice(
        format!(
            "{}#[derive(Debug,Copy,Clone,Decode)]pub enum Position{{Top,Jungle,Middle,Bottom,Support}}
            pub const BASIC_ATTACK_OFFSET:(u32,u32)={};pub const CRITICAL_STRIKE_OFFSET:(u32,u32)={};
            pub const ONHIT_EFFECT_OFFSET:(u32,u32)={};pub const UNCOMPRESSED_MEGA_BLOCK_SIZE:usize={};",
            exported_content,
            record_offsets!(highlight_rust(&invoke_rustfmt(BASIC_ATTACK, 60))),
            record_offsets!(highlight_rust(&invoke_rustfmt(CRITICAL_STRIKE, 60))),
            record_offsets!(ONHIT_EFFECT),
            current_offset,
        )
        .as_bytes(),
    );

    let _ = fs::write(
        cwd!("tutorlolv2_exports/assets/mega_block.txt"),
        &mega_block,
    );

    let mega_block_compressed = encode_brotli_11(mega_block.as_bytes());

    bytes.extend_from_slice(
        format!(
            "pub const MEGA_BLOCK:[u8;{}]=[{}];",
            mega_block_compressed.len(),
            mega_block_compressed.join(",")
        )
        .as_bytes(),
    );

    let _ = fs::write(cwd!("tutorlolv2_exports/src/export_code.rs"), bytes);
}
