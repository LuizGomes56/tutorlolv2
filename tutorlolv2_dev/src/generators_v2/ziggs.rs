use super::*;

impl Generator for Ziggs {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (1, 0, _1));
        ability!(W, (1, 0, _1));
        ability!(E, (1, 0, _1), (1, 1, _2), (1, 2, _3));
        ability!(R, (1, 0, _1), (1, 1, _2));
    }
}