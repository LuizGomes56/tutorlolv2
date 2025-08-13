use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_jax(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
    ability!(w, (0, 0, _1, Min));
    ability!(e, (1, 0, _1Max, Max), (1, 1, Minion1, Min));
    ability!(r, (0, 4, _1Min, Min), (2, 0, _2, Min));
}
