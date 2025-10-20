use super::*;

impl Generator for Yasuo {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(E, (0, 0, _1), (1, 0, _2), (1, 1, _3), (1, 2, _4));
        ability!(R, (1, 0, _1));
    }
}
