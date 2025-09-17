use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_kalista(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min));
    ability!(w, (1, 0, Void, Min), (1, 1, Max, Max));
    ability!(e, (1, 0, Void, Min), (1, 1, _1, Min));
    merge_ability!(W::Void, W::Max);
}
