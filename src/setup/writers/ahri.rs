use super::{Ability, CdnChampion, Champion, HashMap, IterationTarget, get_from_pattern};

pub fn transform(data: CdnChampion) -> Champion {
    let mut abilities = HashMap::<String, Ability>::new();

    get_from_pattern(
        &data.abilities.q[0],
        &mut abilities,
        &[
            (0, 0, "Q1", IterationTarget::MINIMUM),
            (0, 1, "Q_MAX", IterationTarget::MAXIMUM),
        ],
    );
    get_from_pattern(
        &data.abilities.w[0],
        &mut abilities,
        &[
            (1, 0, "W1", IterationTarget::MINIMUM),
            (1, 1, "W2", IterationTarget::MINIMUM),
            (1, 2, "W_MAX", IterationTarget::MAXIMUM),
            (3, 0, "W1_MINION", IterationTarget::MINIMUM),
            (3, 1, "W2_MINION", IterationTarget::MINIMUM),
        ],
    );
    get_from_pattern(
        &data.abilities.e[0],
        &mut abilities,
        &[(0, 0, "E", IterationTarget::MINIMUM)],
    );
    get_from_pattern(
        &data.abilities.r[0],
        &mut abilities,
        &[(0, 0, "R_MIN", IterationTarget::MINIMUM)],
    );

    abilities.insert(String::from("Q2"), abilities.get("Q1").unwrap().clone());
    abilities.insert(
        String::from("R_MAX"),
        abilities.get("R_MIN").unwrap().clone(),
    );
    abilities.get_mut("Q1").unwrap().damage_type = String::from("MAGIC_DAMAGE");
    abilities.get_mut("Q2").unwrap().damage_type = String::from("TRUE_DAMAGE");
    if let Some(ability) = abilities.get_mut("R_MAX") {
        let mut r_max_dmg = Vec::<String>::new();
        for r_str in &ability.minimum_damage {
            r_max_dmg.push(format!("3 * ({})", r_str));
        }
        ability.minimum_damage = Vec::<String>::new();
        ability.maximum_damage = r_max_dmg;
    }

    data.format(abilities)
}
