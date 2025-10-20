use super::*;

impl Generator for Lux {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(E, (2, 0, _1));
        ability!(R, (0, 0, _1));
    }
}