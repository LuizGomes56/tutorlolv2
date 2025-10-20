use super::*;

impl Generator for Yuumi {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1), (1, 0, _2), (2, 0, _3));
        ability!(R, (2, 0, _1), (2, 1, _2), (2, 2, _3));
    }
}
