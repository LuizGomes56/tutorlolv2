use super::*;

// #![stable] "06/18/2025" | "25.11"
// #![unsupported] MINION | MONSTER

#[generator_macros::generator]
pub fn gen_ahri(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, "Q", Target::MINIMUM),);
    ability!(
        w,
        (1, 0, "W", Target::MINIMUM),
        (1, 1, "W1", Target::MINIMUM),
        (1, 2, "W_MAX", Target::MAXIMUM),
        (3, 0, "W_MINION", Target::MINIMUM),
        (3, 1, "W_MINION_MAX", Target::MAXIMUM)
    );
    ability!(e, (0, 0, "E", Target::MINIMUM));
    ability!(r, (0, 0, "R", Target::MINIMUM));

    merge_ability!("W");
    merge_ability!("W_MINION");

    let q_max = merge_damage!(
        5,
        || format!("({}) * MAGIC_MULTIPLIER + ({})", q, q),
        (q, minimum_damage)
    );

    let q_mut_ref = get!(mut "Q");
    q_mut_ref.maximum_damage = q_max;
    q_mut_ref.damage_type = String::from("MIXED");

    let r_max = merge_damage!(3, || format!("3 * ({})", r), (r, minimum_damage));
    get!(mut "R").maximum_damage = r_max;
}
