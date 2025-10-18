use super::*;

// #![preserve]

#[tutorlolv2_macros::generator]
pub fn gen_ambessa(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max),
        (0, 1, _1),
        (0, 0, _2Max),
        (0, 1, _2)
    );
    ability!(w, (0, 0, Void), (1, 0, Max));
    ability!(e, (0, 0, Void), (0, 1, Max));
    ability!(r, (0, 0, Void));
}
