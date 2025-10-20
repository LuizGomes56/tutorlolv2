use super::*;

impl Generator for Vex {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (0, 1, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1), (2, 0, _2), (2, 1, _3));
    }
}