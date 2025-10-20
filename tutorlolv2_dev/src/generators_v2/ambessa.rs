use super::*;

impl Generator for Ambessa {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4));
        ability!(W, (0, 0, _1), (1, 0, _2));
        ability!(E, (0, 0, _1), (0, 1, _2));
        ability!(R, (1, 0, _1));
    }
}