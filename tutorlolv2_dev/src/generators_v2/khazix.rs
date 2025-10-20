use super::*;

impl Generator for Khazix {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (1, 0, _1), (1, 1, _2));
        ability!(W, (0, 0, _1));
        ability!(E, (0, 0, _1));
    }
}