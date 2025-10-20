use super::*;

impl Generator for Naafiri {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (0, 2, _3),
            (0, 3, _4),
            (1, 0, _5),
            (1, 1, _6),
            (1, 2, _7),
            (1, 3, _8)
        );
        ability!(E, (0, 0, _1), (0, 1, _2), (0, 2, _3));
        ability!(R, (0, 0, _1), (0, 1, _2));
    }
}