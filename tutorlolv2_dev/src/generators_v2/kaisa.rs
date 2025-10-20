use super::*;

impl Generator for Kaisa {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (1, 0, _2), (1, 1, _3), (2, 0, _4));
        ability!(W, (0, 0, _1));
    }
}