use super::*;

impl Generator for Kled {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (1, 0, _3),
            (1, 1, _4),
            (0, 0, _5),
            (2, 0, _6),
            (2, 1, _7)
        );
        ability!(W, (1, 0, _1), (2, 0, _2));
        ability!(E, (0, 0, _1), (2, 0, _2));
        ability!(R, (3, 0, _1), (3, 1, _2));
    }
}