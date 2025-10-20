use super::*;

impl Generator for Rell {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 1, _1), (1, 0, _2));
        ability!(E, (1, 0, _1));
        ability!(R, (0, 0, _1), (0, 1, _2));
    }
}