use super::*;

impl Generator for Qiyana {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (1, 0, _3),
            (1, 1, _4),
            (4, 0, _5),
            (4, 1, _6)
        );
        ability!(W, (0, 1, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (1, 0, _1), (1, 1, _2));
    }
}