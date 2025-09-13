use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_akshan(data: CdnChampion) -> Champion {
    ability!(q, (0, 0, Void, Min), (1, 0, _1, Min), (3, 0, _1Max, Max));
    ability!(e, (4, 0, Void, Min));
    ability!(
        r,
        (4, 0, _1, Min),
        (4, 1, _1Max, Max),
        (4, 2, _2, Min),
        (4, 3, _2Max, Min)
    );
}
