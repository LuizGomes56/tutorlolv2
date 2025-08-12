use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_akali(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, None, Min));
    ability!(e, (0, 0, None, Min), (2, 0, _1, Min), (2, 1, _1Max, Max));
    ability!(r, (0, 0, None, Min), (2, 0, _1Max, Max), (2, 1, _1Min, Min));
}
