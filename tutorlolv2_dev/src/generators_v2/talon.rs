use super::*;

impl Generator for Talon {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1), (0, 1, _2));
        ability!(W, (0, 0, _1), (1, 0, _2), (1, 1, _3));
        ability!(R, (0, 0, _1), (2, 0, _2));
    }
}