use super::*;

impl Generator for Galio {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 0, _1), (1, 1, _2), (3, 0, _3), (3, 1, _4));
        ability!(E, (1, 0, _1), (1, 1, _2));
        ability!(R, (1, 0, _1));
    }
}