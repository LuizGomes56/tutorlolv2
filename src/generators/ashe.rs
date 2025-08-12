use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ashe(data: CdnChampion) -> Champion {
    ability!(q, (0, 1, None, Min), (0, 2, Max, Max));
    ability!(w, (0, 1, None, Min));
    ability!(r, (0, 0, None, Min));
}
