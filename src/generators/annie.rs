use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_annie(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, None, Min));
    ability!(w, (0, 0, None, Min));
    ability!(e, (1, 0, None, Min));
    ability!(r, (0, 0, None, Min));
}
