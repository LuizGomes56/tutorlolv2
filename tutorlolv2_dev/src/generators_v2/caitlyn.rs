use super::*;

impl Generator for Caitlyn {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1), (0, 1, _2));
        ability!(W, (2, 0, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (2, 0, _1));
    }
}