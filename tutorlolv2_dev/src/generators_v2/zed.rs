use super::*;

impl Generator for Zed {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1), (0, 1, _2));
        ability!(E, (0, 0, _1));
        ability!(R, (2, 0, _1));
    }
}
