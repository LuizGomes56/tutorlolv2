use super::*;

impl Generator for Viego {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1), (0, 1, _2), (3, 0, _3), (3, 1, _4));
        ability!(W, (2, 0, _1));
        ability!(R, (0, 0, _1));
    }
}