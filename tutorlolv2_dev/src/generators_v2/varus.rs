use super::*;

impl Generator for Varus {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (2, 0, _1), (2, 1, _2), (2, 2, _3), (2, 3, _4));
        ability!(
            W,
            (0, 0, _1),
            (1, 0, _2),
            (1, 1, _3),
            (1, 2, _4),
            (1, 3, _5),
            (2, 0, _6),
            (2, 1, _7)
        );
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1));
    }
}