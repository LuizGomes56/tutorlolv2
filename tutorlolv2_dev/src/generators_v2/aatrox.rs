use super::*;

impl Generator for Aatrox {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (2, 0, _1),
            (2, 1, _2),
            (3, 0, _3),
            (3, 1, _4),
            (4, 0, _5),
            (4, 1, _6),
            (5, 0, _7),
            (5, 1, _8)
        );
        ability!(W, (0, 0, _1), (0, 1, _2), (2, 0, _3));
        ability!(R, (2, 0, _1));
    }
}
