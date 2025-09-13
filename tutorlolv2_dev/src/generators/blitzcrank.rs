use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_blitzcrank(data: CdnChampion) -> Champion {
    let _ = extract_scaled_values("Unimplemented");
    ability!(q, (0, 0, _1Min, Min));
    ability!(r, (0, 0, _1Min, Min), (1, 0, _2Min, Min));
}
