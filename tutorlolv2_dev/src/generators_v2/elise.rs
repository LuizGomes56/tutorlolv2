use super::*;

impl Generator for Elise {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4));
        ability!(W, (1, 0, _1));
    }
}