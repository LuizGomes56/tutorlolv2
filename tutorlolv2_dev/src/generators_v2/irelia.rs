use super::*;

impl Generator for Irelia {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(W, (2, 0, _1), (2, 1, _2));
        ability!(E, (2, 0, _1));
        ability!(R, (0, 0, _1));
    }
}