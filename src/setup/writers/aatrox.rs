use std::collections::HashMap;

use super::{
    Ability, CdnChampion, Champion, IterationTarget, extract_as_linear_range,
    extract_percentage_range, get_from_pattern,
};

fn passive(data: &CdnChampion, map: &mut HashMap<String, Ability>) {
    let passive = data
        .abilities
        .p
        .get(0)
        .expect("Aatrox's passive is missing");

    let passive_effects = passive
        .effects
        .get(0)
        .expect("Aatrox's passive has no Effect[0]")
        .description
        .clone();

    let passive_bounds = extract_percentage_range(&passive_effects)
        .expect("Couldn't extract numeric values for Aatrox's passive");

    let passive_min_dmg = extract_as_linear_range(passive_bounds, 18, "ENEMY_MAX_HEALTH");

    map.insert(String::from("P"), passive.format(passive_min_dmg, vec![]));
}

pub fn transform(data: CdnChampion) -> Champion {
    let mut abilities = HashMap::<String, Ability>::new();

    passive(&data, &mut abilities);
    get_from_pattern(
        &data.abilities.q[0],
        HashMap::from([
            (
                2,
                HashMap::from([
                    (0, (String::from("Q1_MIN"), IterationTarget::MINIMUM)),
                    (1, (String::from("Q1_MAX"), IterationTarget::MAXIMUM)),
                ]),
            ),
            (
                3,
                HashMap::from([
                    (0, (String::from("Q2_MIN"), IterationTarget::MINIMUM)),
                    (1, (String::from("Q2_MAX"), IterationTarget::MAXIMUM)),
                ]),
            ),
            (
                4,
                HashMap::from([
                    (0, (String::from("Q3_MIN"), IterationTarget::MINIMUM)),
                    (1, (String::from("Q3_MAX"), IterationTarget::MAXIMUM)),
                ]),
            ),
        ]),
        &mut abilities,
    );
    get_from_pattern(
        &data.abilities.w[0],
        HashMap::from([
            (
                0,
                HashMap::from([
                    (0, (String::from("W_MIN"), IterationTarget::MINIMUM)),
                    (1, (String::from("W_MINION"), IterationTarget::MINIMUM)),
                ]),
            ),
            (
                2,
                HashMap::from([(0, (String::from("W_MAX"), IterationTarget::MAXIMUM))]),
            ),
        ]),
        &mut abilities,
    );

    data.format(abilities)
}
