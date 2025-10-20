use super::*;

impl Generator for Gwen {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (1, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (1, 3, _4),
            (2, 0, _5),
            (2, 1, _6),
            (2, 2, _7),
            (2, 3, _8)
        );
        ability!(E, (0, 0, _1));
        ability!(
            R,
            (0, 0, _1),
            (0, 1, _2),
            (2, 0, _3),
            (2, 1, _4),
            (2, 2, _5)
        );
    }
}