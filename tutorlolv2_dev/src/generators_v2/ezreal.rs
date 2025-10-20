use super::*;

impl Generator for Ezreal {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 0, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1), (1, 0, _2));
    }
}