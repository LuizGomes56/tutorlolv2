use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_nautilus(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min));
    ability!(w, (1, 0, _1Min));
    ability!(
        e,
        (0, 0, _1Min),
        (0, 1, _2Max),
        (0, 2, _3Min),
        (1, 0, Monster1),
        (1, 1, Monster2),
        (1, 2, Monster3)
    );
    ability!(r, (0, 0, _1Min), (1, 0, _2Max));
}
