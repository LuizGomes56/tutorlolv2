use super::*;

impl Generator for Kennen {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (0, 0, _1), (1, 0, _2));
        ability!(E, (0, 0, _1), (0, 1, _2));
        ability!(R, (1, 0, _1), (2, 0, _2));
    }
}