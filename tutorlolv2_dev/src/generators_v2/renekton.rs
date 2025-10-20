use super::*;

impl Generator for Renekton {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (0, 1, _2));
        ability!(W, (0, 0, _1), (0, 1, _2), (2, 0, _3));
        ability!(E, (0, 0, _1), (2, 0, _2), (4, 0, _3), (4, 1, _4));
        ability!(R, (1, 0, _1), (1, 1, _2));
    }
}