use super::*;

impl Generator for Tryndamere {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (0, 0, _1));
        ability!(E, (0, 0, _1));
    }
}