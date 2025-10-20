use super::*;

impl Generator for Sett {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (0, 1, _2));
        ability!(W, (2, 0, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (1, 0, _1), (1, 1, _2));
    }
}