use super::*;

impl Generator for Lillia {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (2, 0, _1), (2, 1, _2));
        ability!(W, (0, 0, _1), (0, 1, _2), (1, 0, _3), (1, 1, _4));
        ability!(E, (0, 0, _1));
        ability!(R, (1, 0, _1));
    }
}