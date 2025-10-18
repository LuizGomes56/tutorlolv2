use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_aurelionsol(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1),
        (0, 1, _2),
        (0, 2, _2Max),
        (5, 0, _3)
    );
    ability!(w, (0, 0, Void));
    ability!(e, (0, 0, Void), (0, 1, Max));
    ability!(r, (0, 0, Void), (0, 0, _1), (1, 0, _2));
}
