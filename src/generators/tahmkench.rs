use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_tahmkench(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, _1Min, Min));
    ability!(w, (2, 1, _1Min, Min));
    ability!(e, (1, 0, _1Min, Min), (1, 1, _2Max, Max));
    ability!(r, (0, 0, _1Min, Min));
}
