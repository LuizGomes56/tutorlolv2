use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_aurelionsol(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1, Min),
        (0, 1, _2, Min),
        (0, 2, _2Max, Max),
        (5, 0, _3, Min)
    );
    ability!(w, (0, 0, None, Min));
    ability!(e, (0, 0, None, Min), (0, 1, Max, Max));
    ability!(r, (0, 0, None, Min), (0, 0, _1, Min), (1, 0, _2, Min));
}
