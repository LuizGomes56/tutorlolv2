use super::*;

impl Generator for Thresh {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (1, 0, _1));
        ability!(E, (0, 0, _1), (0, 1, _2), (1, 0, _3));
        ability!(R, (0, 0, _1));
    }
}