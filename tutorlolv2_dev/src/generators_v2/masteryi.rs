use super::*;

impl Generator for MasterYi {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (2, 0, _1),
            (2, 1, _2),
            (2, 2, _3),
            (2, 3, _4),
            (2, 4, _5),
            (2, 5, _6),
            (2, 6, _7)
        );
        ability!(W, (1, 0, _1), (1, 1, _2));
        ability!(E, (0, 0, _1));
    }
}