use super::*;

impl Generator for Malphite {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 0, _1), (2, 0, _2));
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1));
    }
}