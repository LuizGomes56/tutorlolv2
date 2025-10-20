use super::*;

impl Generator for KogMaw {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (1, 0, _1));
        ability!(W, (0, 1, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1), (0, 1, _2));
    }
}