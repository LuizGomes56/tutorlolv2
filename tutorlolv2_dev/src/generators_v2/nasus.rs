use super::*;

impl Generator for Nasus {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(E, (0, 0, _1), (1, 0, _2), (1, 1, _3));
        ability!(R, (1, 0, _1), (1, 1, _2));
    }
}