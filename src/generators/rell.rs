use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_rell(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min), (0, 0, _2, Min));
    ability!(e, (1, 0, _1, Min));
    ability!(r, (0, 0, _1Min, Min), (0, 1, _2Max, Max));
}
