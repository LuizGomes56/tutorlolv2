use super::*;

impl Generator for Morgana {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 0, _1), (1, 1, _2), (1, 2, _3), (1, 3, _4));
        ability!(R, (0, 0, _1), (0, 1, _2));
    }
}