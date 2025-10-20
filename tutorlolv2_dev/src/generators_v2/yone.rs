use super::*;

impl Generator for Yone {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (0, 0, _1), (0, 1, _2), (0, 2, _3));
        ability!(E, (1, 0, _1));
        ability!(R, (1, 0, _1), (1, 1, _2), (1, 2, _3));
    }
}