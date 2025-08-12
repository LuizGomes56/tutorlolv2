use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_azir(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, None, Min));
    ability!(w, (3, 0, None, Min));
    ability!(e, (1, 0, None, Min));
    ability!(r, (1, 0, None, Min));
}
