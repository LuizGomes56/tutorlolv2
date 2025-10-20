use super::*;

impl Generator for Gnar {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1), (0, 1, _2), (0, 0, _3));
        ability!(W, (1, 0, _1), (0, 0, _2));
        ability!(E, (1, 0, _1), (0, 0, _2));
        ability!(R, (1, 0, _1), (2, 0, _2));
    }
}