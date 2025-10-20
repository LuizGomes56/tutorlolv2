use super::*;

impl Generator for Udyr {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (0, 1, _2), (1, 1, _3));
        ability!(R, (1, 0, _1), (1, 1, _2));
    }
}