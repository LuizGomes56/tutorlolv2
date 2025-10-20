use super::*;

impl Generator for Malzahar {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (2, 0, _1), (2, 1, _2));
        ability!(E, (0, 0, _1), (0, 1, _2));
        ability!(R, (0, 0, _1), (0, 1, _2), (2, 0, _3), (2, 1, _4));
    }
}