use super::*;

impl Generator for Syndra {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (2, 0, _1), (3, 0, _2), (3, 1, _3));
        ability!(E, (0, 0, _1));
        ability!(R, (1, 0, _1), (1, 1, _2), (1, 2, _3));
    }
}