use super::*;

impl Generator for Nunu {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (1, 0, _1), (2, 0, _2));
        ability!(W, (2, 0, _1), (2, 1, _2), (3, 0, _3), (3, 1, _4));
        ability!(E, (0, 0, _1), (0, 1, _2), (2, 0, _3), (3, 0, _4));
        ability!(R, (2, 0, _1));
    }
}