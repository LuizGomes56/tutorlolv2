use std::collections::HashMap;

use super::{Ability, CdnChampion, Champion, IterationTarget, get_from_pattern};

pub fn transform(data: CdnChampion) -> Champion {
    let mut abilities = HashMap::<String, Ability>::new();

    get_from_pattern(
        &data.abilities.q[0],
        HashMap::from([(
            0,
            HashMap::from([
                (0, (String::from("Q1"), IterationTarget::MINIMUM)),
                (1, (String::from("Q_MAX"), IterationTarget::MAXIMUM)),
            ]),
        )]),
        &mut abilities,
    );
    get_from_pattern(
        &data.abilities.w[0],
        HashMap::from([
            (
                1,
                HashMap::from([
                    (0, (String::from("W1"), IterationTarget::MINIMUM)),
                    (1, (String::from("W2"), IterationTarget::MINIMUM)),
                    (2, (String::from("W_MAX"), IterationTarget::MAXIMUM)),
                ]),
            ),
            (
                3,
                HashMap::from([
                    (0, (String::from("W1_MINION"), IterationTarget::MINIMUM)),
                    (1, (String::from("W2_MINION"), IterationTarget::MINIMUM)),
                ]),
            ),
        ]),
        &mut abilities,
    );
    get_from_pattern(
        &data.abilities.e[0],
        HashMap::from([(
            0,
            HashMap::from([(0, (String::from("E"), IterationTarget::MINIMUM))]),
        )]),
        &mut abilities,
    );
    get_from_pattern(
        &data.abilities.r[0],
        HashMap::from([(
            0,
            HashMap::from([(0, (String::from("R_MIN"), IterationTarget::MINIMUM))]),
        )]),
        &mut abilities,
    );

    for (key, _) in abilities.iter() {
        println!("{}", key);
    }

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
