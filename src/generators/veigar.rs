use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_veigar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, _1Min, Min));
    ability!(w, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Max, Max), (0, 1, Minion1, Min));
}
