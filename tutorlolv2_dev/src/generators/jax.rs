use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_jax(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min));
    ability!(w, (0, 0, _1));
    ability!(e, (1, 0, _1Max), (1, 1, Minion1));
    ability!(r, (0, 4, _1Min), (2, 0, _2));
}
