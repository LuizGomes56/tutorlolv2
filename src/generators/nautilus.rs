use super::*;

// #![preserve]

#[generator_macros::generator]
pub fn gen_nautilus(data: CdnChampion) -> Champion {
    ability!(q, (1, 0, _1Min, Min));
    ability!(w, (1, 0, _1Min, Min));
    ability!(
        e,
        (0, 0, _1Min, Min),
        (0, 1, _2Max, Max),
        (0, 2, _3Min, Min),
        (1, 0, Monster1, Min),
        (1, 1, Monster2, Min),
        (1, 2, Monster3, Min)
    );
    ability!(r, (0, 0, _1Min, Min), (1, 0, _2Max, Max));
}
