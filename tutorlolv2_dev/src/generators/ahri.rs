use super::*;

// #![stable] "08/07/2025" | "25.15"

#[tutorlolv2_macros::generator]
pub fn gen_ahri(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(
        w,
        (1, 0, Void, Min),
        (1, 1, _1, Min),
        (1, 2, Max, Max),
        (3, 0, Minion, Max),
        (3, 1, MinionMax, Max)
    );
    ability!(e, (0, 1, Void, Min));
    ability!(r, (0, 0, Void, Min));

    let q_max = merge_damage!(
        5,
        |(q,)| format!("({}) * MAGIC_MULTIPLIER + ({})", q, q),
        Q::Void,
    );

    insert!(Q::Max, get!(Q::Void).clone());
    let q_mut_ref = get!(Q::Max);
    q_mut_ref.damage = q_max;
    q_mut_ref.damage_type = DamageType::Mixed;

    insert!(R::Max, get!(R::Void).clone());
    let r_max = merge_damage!(3, |(r,)| format!("3 * ({})", r), R::Void);
    get!(mut R::Void).damage = r_max;
}
