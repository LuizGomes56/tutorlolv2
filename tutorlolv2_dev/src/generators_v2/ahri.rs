use super::*;

impl Generator for Ahri {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1), (0, 1, _2));
        ability!(
            W,
            (1, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (3, 0, _4),
            (3, 1, _5)
        );
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1));
    }
}