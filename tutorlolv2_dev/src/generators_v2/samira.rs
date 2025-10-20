use super::*;

impl Generator for Samira {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 0, _1), (1, 1, _2));
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1), (0, 1, _2), (0, 2, _3), (0, 3, _4));
    }
}