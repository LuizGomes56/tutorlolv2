use super::*;

impl Generator for Sion {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (0, 2, _3),
            (2, 0, _4),
            (2, 1, _5),
            (2, 2, _6),
            (2, 3, _7)
        );
        ability!(W, (2, 0, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (3, 0, _1), (3, 1, _2));
    }
}