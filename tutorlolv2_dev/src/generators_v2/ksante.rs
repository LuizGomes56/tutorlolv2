use super::*;

impl Generator for KSante {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(
            W,
            (2, 0, _1),
            (2, 1, _2),
            (3, 0, _3),
            (3, 1, _4),
            (3, 2, _5)
        );
        ability!(R, (0, 0, _1), (1, 0, _2), (1, 1, _3));
    }
}