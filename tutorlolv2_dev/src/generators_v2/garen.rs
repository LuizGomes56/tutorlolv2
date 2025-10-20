use super::*;

impl Generator for Garen {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (1, 0, _1));
        ability!(W, (2, 0, _1));
        ability!(E, (0, 0, _1), (2, 0, _2));
        ability!(R, (0, 0, _1));
    }
}