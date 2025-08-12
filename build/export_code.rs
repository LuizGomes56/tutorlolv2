use super::*;

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

pub async fn export_code() {
    let mut mega_block = String::with_capacity(1 << 24);

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
        "pub static CHAMPION_ABILITIES: [&'static [(usize, usize)]; {}] = [",
        champions.len(),
    );
    let mut internal_items = String::from(
        "pub static INTERNAL_ITEMS: ::phf::Map<u32, &'static CachedItem> = ::phf::phf_map! {",
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
    let mut internal_runes = String::from(
        "pub static INTERNAL_RUNES: phf::Map<u32, &'static CachedRune> = phf::phf_map! {",
    );
    let mut internal_runes_content = String::new();

    let mut current_offset = 0usize;

    macro_rules! record_offsets {
        ($value:expr) => {{
            let start = current_offset;
            mega_block.push_str(&$value);
            let end = current_offset + $value.len();
            current_offset = end;
            format!("({}, {})", start, end)
        }};
        ($field:ident, $value:expr) => {{
            let start = current_offset;
            mega_block.push_str(&$value);
            let end = current_offset + $value.len();
            current_offset = end;
            $field.push_str(&format!("({}, {}),", start, end));
        }};
    }

    for (champion_id, champion_detail) in champions.into_iter() {
        internal_champions.push_str(&format!("&{},", champion_id.to_uppercase()));
        champion_id_to_name.push_str(&format!("\"{}\",", champion_detail.champion_name));
        record_offsets!(champion_formulas, champion_detail.champion_formula);
        record_offsets!(champion_generator, champion_detail.generator);

        champion_abilities.push('[');
        for (_, ability_formula) in &champion_detail.highlighted_abilities {
            record_offsets!(champion_abilities, ability_formula);
        }
        champion_abilities.push_str("],");
        internal_champions_content.push_str(&champion_detail.constdecl);
    }

    internal_champions.push_str("];");
    champion_id_to_name.push_str("];");
    champion_formulas.push_str("];");
    champion_generator.push_str("];");
    champion_abilities.push_str("];");

    tokio::task::spawn_blocking(move || {
        fs::write("internal_comptime/src/data/champions.rs", {
            let mut s = String::with_capacity(
                USE_SUPER.len() + internal_champions_content.len() + internal_champions.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(&internal_champions_content);
            s.push_str(&internal_champions);
            s.push_str(&BASIC_ATTACK);
            s.push_str(&CRITICAL_STRIKE);
            s
        })
    });

    let mut internal_damaging_items_len = 0usize;
    let mut internal_simulated_items_len = 0usize;
    for (item_id, item_detail) in items.into_iter() {
        internal_items_content.push_str(&item_detail.constdecl);
        internal_items.push_str(&format!("{}u32 => &ITEM_{},", item_id, item_id));
        if item_detail.is_simulated {
            internal_simulated_items.push_str(&format!("{}u32,", item_id));
            internal_simulated_items_len += 1;
        }
        if item_detail.is_damaging {
            internal_damaging_items.push_str(&format!("{}u32,", item_id));
            internal_damaging_items_len += 1;
        }
        record_offsets!(item_formulas, item_detail.item_formula);
        item_descriptions.push_str(&format!("{}u32 => {},", item_id, item_detail.description));
        item_id_to_name.push_str(&format!("\"{}\",", item_detail.item_name));
    }

    internal_items.push_str("};");
    internal_simulated_items.push_str(");");
    internal_damaging_items.push_str(");");
    item_id_to_name.push_str("];");
    item_formulas.push_str("];");
    item_descriptions.push_str("];");

    tokio::task::spawn_blocking(move || {
        fs::write("internal_comptime/src/data/items.rs", {
            let mut s = String::with_capacity(
                internal_items.len()
                    + internal_items_content.len()
                    + internal_simulated_items.len()
                    + internal_damaging_items.len()
                    + USE_SUPER.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(&internal_items);
            s.push_str(&internal_items_content);
            s.push_str(&internal_simulated_items);
            s.push_str(&internal_damaging_items);
            s.push_str(&format!(
                "pub const SIZE_SIMULATED_ITEMS: usize = {};
            pub const SIZE_DAMAGING_ITEMS: usize = {};",
                internal_simulated_items_len, internal_damaging_items_len
            ));
            s
        })
    });

    let internal_damaging_runes = format!(
        "pub static DAMAGING_RUNES: phf::Set<u32> = phf::phf_set!({});",
        runes
            .iter()
            .map(|(rune_id, _)| format!("{}u32", rune_id))
            .collect::<Vec<String>>()
            .join(",")
    );

    let internal_damaging_runes_len = runes.len();
    for (rune_id, rune_detail) in runes.into_iter() {
        internal_runes.push_str(&format!("{}u32 => &RUNE_{},", rune_id, rune_id));
        internal_runes_content.push_str(&rune_detail.constdecl);
        rune_id_to_name.push_str(&format!("\"{}\",", rune_detail.rune_name));
        record_offsets!(rune_formulas, rune_detail.rune_formula);
    }

    internal_runes.push_str("};");
    rune_id_to_name.push_str("];");
    rune_formulas.push_str("];");

    tokio::task::spawn_blocking(move || {
        fs::write("internal_comptime/src/data/runes.rs", {
            let mut s = String::with_capacity(
                internal_runes.len()
                    + internal_runes_content.len()
                    + USE_SUPER.len()
                    + internal_damaging_runes.len(),
            );
            s.push_str(USE_SUPER);
            s.push_str(&internal_runes);
            s.push_str(&internal_runes_content);
            s.push_str(&internal_damaging_runes);
            s.push_str(&format!(
                "pub const SIZE_DAMAGING_RUNES: usize = {};",
                internal_damaging_runes_len
            ));
            s
        })
    });

    let mut bytes = Vec::from(b"use crate::*;");

    bytes.extend_from_slice(
        format!(
            "
            pub static BASIC_ATTACK_OFFSET: (usize, usize) = {};
            pub static CRITICAL_STRIKE_OFFSET: (usize, usize) = {};
            pub static ONHIT_EFFECT_OFFSET: (usize, usize) = {};
            pub static UNCOMPRESSED_MEGA_BLOCK_SIZE: usize = {};
            ",
            record_offsets!(highlight(&invoke_rustfmt(&BASIC_ATTACK, 60))),
            record_offsets!(highlight(&invoke_rustfmt(&CRITICAL_STRIKE, 60))),
            record_offsets!(ONHIT_EFFECT),
            current_offset
        )
        .as_bytes(),
    );

    let _ = fs::write("comptime_exports/mega_block.txt", &mega_block);

    let mega_block_compressed = compress_bytes!(mega_block.as_bytes());

    bytes.extend_from_slice(
        format!(
            "pub static MEGA_BLOCK: [u8; {}] = [{}];",
            mega_block_compressed.len(),
            mega_block_compressed.join(",")
        )
        .as_bytes(),
    );

    let _ = fs::write("comptime_exports/export_code.txt", bytes);
}
