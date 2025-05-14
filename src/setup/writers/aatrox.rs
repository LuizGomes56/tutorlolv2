use super::{
    Ability, CdnChampion, Champion, HashMap, IterationTarget, get_from_pattern, get_passive_damage,
};

pub fn transform(data: CdnChampion) -> Champion {
    let mut abilities = HashMap::<String, Ability>::new();

    get_passive_damage(
        &data,
        (0, 0),
        Some("ENEMY_MAX_HEALTH"),
        None,
        &IterationTarget::MINIMUM,
        "P",
        &mut abilities,
    );
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
