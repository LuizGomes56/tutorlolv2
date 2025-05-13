use super::{
    Ability, CdnChampion, Champion, HashMap, IterationTarget, assign_as_linear_range,
    extract_passive_bounds, extract_range_values, get_from_pattern,
};

pub fn transform(data: CdnChampion) -> Champion {
    let mut abilities = HashMap::<String, Ability>::new();

    let (passive, passive_bounds) = extract_passive_bounds(&data, (0, 0));

    let passive_min_dmg = assign_as_linear_range(passive_bounds, 18, Some("ENEMY_MAX_HEALTH"));

    abilities.insert(String::from("P"), passive.format(passive_min_dmg, vec![]));

    get_from_pattern(
        &data.abilities.q[0],
        &mut abilities,
        &[
            (2, 0, "Q1_MIN", IterationTarget::MINIMUM),
            (2, 1, "Q1_MAX", IterationTarget::MAXIMUM),
            (3, 0, "Q2_MIN", IterationTarget::MINIMUM),
            (3, 1, "Q2_MAX", IterationTarget::MAXIMUM),
            (4, 0, "Q3_MIN", IterationTarget::MINIMUM),
            (4, 1, "Q3_MAX", IterationTarget::MAXIMUM),
        ],
    );
    get_from_pattern(
        &data.abilities.w[0],
        &mut abilities,
        &[
            (0, 0, "W_MIN", IterationTarget::MINIMUM),
            (0, 1, "W_MINION", IterationTarget::MINIMUM),
            (2, 0, "W_MAX", IterationTarget::MAXIMUM),
        ],
    );

    data.format(abilities)
}
