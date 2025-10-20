use super::*;

impl Generator for Nami {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 2, _1), (1, 3, _2));
        ability!(E, (0, 0, _1), (0, 1, _2));
        ability!(R, (0, 0, _1));
    }
}