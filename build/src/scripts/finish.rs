use {
    crate::{
        CPARSER, MayFail,
        generators::{parser::Parser, utils::Tag},
        scripts::batch::{FmtArgs, FmtOutput},
    },
    std::{
        collections::{BTreeMap, BTreeSet},
        ops::Range,
    },
    tutorlolv2_types::{AbilityId, AttackType, DamageIndex},
    tutorlolv2_wiki::JsonRead,
};

pub fn finish_champions(target: &str, variable: &mut String, value: &mut [FmtOutput]) {
    value.sort_by(|a, b| match &a.json.meta {
        v if let Ok(ability_a) = serde_json::from_value::<AbilityId>(v.clone())
            && let Ok(ability_b) = serde_json::from_value::<AbilityId>(b.json.meta.clone()) =>
        {
            ability_a.cmp(&ability_b)
        }
        _ => a.json.target.cmp(&b.json.target),
    });

    let ranges = value
        .iter()
        .map(|FmtOutput { range, .. }| format!("{range:?}"))
        .collect::<Vec<_>>()
        .join(",");

    let push = match target {
        "formula" | "generator" | "debug" | "json" => format!("{ranges},"),
        "ability" | "closure" => format!("&[{ranges},],"),
        _ => panic!("Unknown target set to fmt_args: {target}"),
    };

    variable.push_str(&push);
}

pub fn finish_items_or_runes(target: &str, variable: &mut String, value: &mut [FmtOutput]) {
    value.sort_by(|a, b| match &a.json.meta {
        v if let Ok((ata, dia)) =
            serde_json::from_value::<(AttackType, DamageIndex)>(v.clone())
            && let Ok((atb, dib)) =
                serde_json::from_value::<(AttackType, DamageIndex)>(b.json.meta.clone()) =>
        {
            ata.cmp(&atb).then(dia.cmp(&dib))
        }
        _ => a.json.target.cmp(&b.json.target),
    });

    let push = match target {
        "formula" | "generator" | "debug" | "json" => {
            value
                .iter()
                .map(|FmtOutput { range, .. }| format!("{range:?}"))
                .collect::<Vec<_>>()
                .join(",")
                + ","
        }
        "closure" => {
            let mut ranges: [[Range<usize>; 2]; 2] =
                core::array::from_fn(|_| core::array::from_fn(|_| 0..0));

            for FmtOutput {
                range,
                json: FmtArgs { meta, .. },
                ..
            } in value
            {
                let (attack_type, damage_index) =
                    serde_json::from_value::<(AttackType, DamageIndex)>(meta.clone()).unwrap();

                ranges[attack_type as usize][damage_index as usize] = range.clone();
            }

            format!("{ranges:?},")
        }
        _ => panic!("Unknown target set to fmt_args: {target}"),
    };

    variable.push_str(&push);
}

pub fn champion_aliases() -> MayFail<Option<BTreeMap<String, Vec<String>>>> {
    let map = CPARSER.map();
    let languages =
        BTreeMap::<String, BTreeSet<String>>::from_file("cache/riot/champion_languages.json")?;

    let alias = map
        .keys()
        .map(|champion_id| {
            (
                champion_id.clone(),
                languages
                    .get(champion_id)
                    .cloned()
                    .unwrap_or_else(|| {
                        eprintln!("languages[{champion_id}] does not exist");
                        Default::default()
                    })
                    .iter()
                    .cloned()
                    .chain(
                        (champion_id == "Gnar")
                            .then_some("Mega Gnar".into())
                            .into_iter(),
                    )
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    Ok(Some(alias))
}

pub fn eval_abilities(arms: &str, _: Tag) -> String {
    let recommendations = get_recommendations().unwrap_or_default();

    format!(
        "pub const fn ability_const_eval(
            champion_id: ChampionId,
            ctx: &Ctx,
            kind: AbilityId
        ) -> f32 {{
            match champion_id {{{arms}}}
        }}

        {recommendations}"
    )
}

pub fn eval_items_or_runes(arms: &str, tag: Tag) -> String {
    format!(
        "pub const fn {ltag}_const_eval(
                {ltag}_id: {enum_name},
                ctx: &Ctx,
                attack_type: AttackType
            ) -> [f32; 2] {{
                match {ltag}_id {{{arms}}}
            }}",
        ltag = tag.singular().to_lowercase(),
        enum_name = tag.enum_name()
    )
}

pub fn get_recommendations() -> MayFail<String> {
    let enum_ids = ["ItemId", "RuneId"];
    let declaration = ["RECOMMENDED_ITEMS", "RECOMMENDED_RUNES"];

    let mut globals = core::array::from_fn::<_, 2, _>(|i| {
        let enumv = enum_ids[i];
        let var = declaration[i];
        format!("pub static {var}: [[&[crate::{enumv}]; 5]; ChampionId::VARIANTS] = [")
    });

    let json = BTreeMap::<String, BTreeMap<String, [BTreeSet<String>; 2]>>::from_file(
        "internal/scraper/data.json",
    )
    .unwrap_or_default();

    if json.is_empty() {
        return Ok(enum_ids
            .iter()
            .zip(declaration)
            .map(|(enumv, var)| {
                format!("pub static {var}: [[&[crate::{enumv}]; 5]; ChampionId::VARIANTS] = [[&[]; _]; _];")
            })
            .collect::<String>());
    }

    let push_end = |globals: &mut [String; 2], str| {
        for value in globals.each_mut() {
            value.push_str(str);
        }
    };

    for data in json.values() {
        push_end(&mut globals, "[");
        for recommendations in data.values() {
            for (i, value) in core::array::from_fn::<_, 2, _>(|j| {
                let venum = enum_ids[j];
                let result = recommendations[j]
                    .iter()
                    .map(|element| format!("{venum}::{element}"))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("&[{result}]")
            })
            .into_iter()
            .enumerate()
            {
                globals[i].push_str(&format!("{value},"));
            }
        }
        push_end(&mut globals, "],");
    }

    push_end(&mut globals, "];");
    Ok(globals.concat())
}
