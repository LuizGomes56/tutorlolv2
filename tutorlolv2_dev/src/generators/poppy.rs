use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_poppy(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, Monster1, Min),
        (0, 1, Monster2, Min),
        (0, 2, _1Min, Min),
        (1, 1, Monster3, Min),
        (1, 2, _2Max, Max)
    );
    ability!(w, (0, 0, _1Min, Min));
    ability!(e, (0, 0, _1Min, Min), (1, 1, _2Max, Max));
    ability!(r, (1, 0, _1Max, Max), (3, 0, _2Min, Min));
}
