use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_ambessa(data: CdnChampion) -> Champion {
    ability!(
        q,
        (0, 0, _1Max, Max),
        (0, 1, _1, Min),
        (0, 0, _2Max, Max),
        (0, 1, _2, Min)
    );
    ability!(w, (0, 0, None, Min), (1, 0, Max, Max));
    ability!(e, (0, 0, None, Min), (0, 1, Max, Max));
    ability!(r, (0, 0, None, Min));
}
