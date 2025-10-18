use super::*;

// #![stable] "08/07/2025" | "25.15"

#[tutorlolv2_macros::generator]
pub fn gen_ahri(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void));
    ability!(
        w,
        (1, 0, Void),
        (1, 1, _1),
        (1, 2, Max),
        (3, 0, Minion),
        (3, 1, MinionMax)
    );
    ability!(e, (0, 1, Void));
    ability!(r, (0, 0, Void));

    let q_max = merge_damage!(
        5,
        |(q,)| format!("({}) * MAGIC_MULTIPLIER + ({})", q, q),
        Q::Void,
    );

    let q_mut_ref = clone_to!(Q::Void => Q::Max);
    q_mut_ref.damage = q_max;
    q_mut_ref.damage_type = DamageType::Mixed;

    clone_to!(R::Void => R::Max);
    let r_max = merge_damage!(3, |(r,)| format!("3 * ({})", r), R::Void);
    get!(mut R::Max).damage = r_max;

    merge_with![
        (Q::Void, Q::Max),
        (W::Void, W::Max),
        (W::Minion, W::MinionMax),
        (R::Void, R::Max)
    ];
}
