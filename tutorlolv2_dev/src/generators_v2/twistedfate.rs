use super::*;

impl Generator for TwistedFate {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (2, 0, _1), (3, 0, _2), (4, 0, _3));
        ability!(E, (0, 1, _1));
    }
}