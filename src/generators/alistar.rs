use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_alistar(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, None, Min));
    ability!(w, (0, 0, None, Min));
    ability!(e, (0, 0, None, Min), (0, 1, Max, Max));
    ability!(r, (0, 0, None, Min));
}
