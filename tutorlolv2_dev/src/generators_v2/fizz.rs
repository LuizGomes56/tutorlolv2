use super::*;

impl Generator for Fizz {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (0, 0, _1), (0, 1, _2), (1, 0, _3), (2, 1, _4));
        ability!(E, (1, 0, _1), (0, 0, _2));
        ability!(R, (1, 0, _1), (2, 0, _2), (3, 0, _3));
    }
}