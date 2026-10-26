use {
    crate::{
        ENGINE_CFG, MayFail,
        generators::{
            parser::{Parser, champions::ChampionParser},
            utils::Tag,
        },
    },
    std::collections::{BTreeMap, BTreeSet},
    tutorlolv2_wiki::JsonRead,
};

impl ChampionParser {
    pub fn aliases(
        &self,
        root: &std::path::Path,
    ) -> MayFail<Option<BTreeMap<String, Vec<String>>>> {
        let map = self.map();

        let languages = BTreeMap::<String, BTreeSet<String>>::from_file(
            root.join("cache/riot/champion_languages.json"),
        )?;

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
                        .chain((champion_id == "Gnar").then_some("Mega Gnar".into()))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();

        Ok(Some(alias))
    }

    pub fn recommendations(&self, root: &std::path::Path) -> MayFail<String> {
        let path = root.join("internal/scraper/data.json");

        let json = match std::fs::read(&path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|e| format!("Invalid recommendations {path:?}: {e}"))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Recommendations::new(),
            Err(e) => return Err(format!("Reading recommendations {path:?}: {e}").into()),
        };

        recommendations_code(self.keys(), &json)
    }
}

pub type Recommendations = BTreeMap<String, BTreeMap<String, [BTreeSet<String>; 2]>>;

pub fn eval_abilities(arms: &str, recommendations: &str) -> String {
    format!(
        "{ENGINE_CFG} pub const fn ability_const_eval(
            champion_id: ChampionId,
            ctx: &tutorlolv2_types::Ctx,
            kind: tutorlolv2_types::AbilityId
        ) -> f32 {{
            match champion_id {{{arms}}}
        }}

        {recommendations}"
    )
}

pub fn eval_items_or_runes(arms: &str, tag: Tag) -> String {
    format!(
        "{ENGINE_CFG} pub const fn {ltag}_const_eval(
            {ltag}_id: {enum_name},
            ctx: &tutorlolv2_types::Ctx,
            attack_type: tutorlolv2_types::AttackType
        ) -> [f32; 2] {{
            match {ltag}_id {{{arms}}}
        }}",
        ltag = tag.enum_name().trim_end_matches("Id").to_lowercase(),
        enum_name = tag.enum_name()
    )
}

pub fn recommendations_code<'a>(
    champions: impl Iterator<Item = &'a str>,
    json: &Recommendations,
) -> MayFail<String> {
    use strum::VariantArray;
    use tutorlolv2_types::Position;

    let mut tables = [String::new(), String::new()];

    for champion in champions {
        let mut roles = BTreeMap::new();

        if let Some(data) = json.get(champion) {
            for (role, recommendations) in data {
                let position = match role.to_ascii_lowercase().as_str() {
                    "top" | "0" => Position::Top,
                    "jungle" | "1" => Position::Jungle,
                    "mid" | "middle" | "2" => Position::Middle,
                    "adc" | "bottom" | "3" => Position::Bottom,
                    "support" | "4" => Position::Support,
                    _ => {
                        return Err(format!(
                            "Unknown recommendation position {role:?} for {champion}"
                        )
                        .into());
                    }
                };

                if roles.insert(position, recommendations).is_some() {
                    return Err(format!(
                        "Duplicate recommendation position {position} for {champion}"
                    )
                    .into());
                }
            }
        }

        for (i, enum_name) in [Tag::Items.enum_name(), Tag::Runes.enum_name()]
            .into_iter()
            .enumerate()
        {
            tables[i].push('[');

            for position in Position::VARIANTS {
                let elements = roles
                    .get(position)
                    .map(|recs| {
                        recs[i]
                            .iter()
                            .map(|id| {
                                // Reject malformed identifiers here with the source entity in the error.
                                syn::parse_str::<syn::Ident>(id).map_err(|e| {
                                    format!("Invalid recommendation {id:?} for {champion}: {e}")
                                })?;

                                Ok(format!("crate::libgen::{enum_name}::{id}"))
                            })
                            .collect::<MayFail<Vec<_>>>()
                    })
                    .transpose()?
                    .unwrap_or_default();

                tables[i].push_str(&format!("&[{}],", elements.join(",")));
            }

            tables[i].push_str("],");
        }
    }

    Ok([
        (Tag::Items.enum_name(), "RECOMMENDED_ITEMS"),
        (Tag::Runes.enum_name(), "RECOMMENDED_RUNES"),
    ]
    .into_iter()
    .zip(tables)
    .map(|((enum_name, name), rows)| {
        let pos_count = "<tutorlolv2_types::Position as strum::EnumCount>::COUNT";
        let champions = "<ChampionId as strum::EnumCount>::COUNT";

        format!(
            "pub static {name}: [[&[crate::{enum_name}]; {pos_count}]; {champions}] = [{rows}];"
        )
    })
    .collect())
}
