use super::*;

// #![stable] "08/07/2025" | "25.15"

#[generator_macros::generator]
pub fn gen_ahri(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, None, Min));
    ability!(
        w,
        (1, 0, None, Min),
        (1, 1, _1, Min),
        (1, 2, Max, Max),
        (3, 0, Minion, Max),
        (3, 1, MinionMax, Max)
    );
    ability!(e, (0, 1, None, Min));
    ability!(r, (0, 0, None, Min));

    merge_ability!((W, None), (W, Max));
    merge_ability!((W, Minion), (W, MinionMax));

    let q_max = merge_damage!(
        5,
        |(q,)| format!("({}) * MAGIC_MULTIPLIER + ({})", q, q),
        ((Q, None), minimum_damage)
    );

    let q_mut_ref = get!(mut (Q, None));
    q_mut_ref.maximum_damage = q_max;
    q_mut_ref.damage_type = DamageType::Mixed.to_string();

    let r_max = merge_damage!(
        3,
        |(r,)| format!("3 * ({})", r),
        ((R, None), minimum_damage)
    );
    get!(mut (R, None)).maximum_damage = r_max;
}
