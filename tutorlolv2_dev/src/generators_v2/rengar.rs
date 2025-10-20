use super::*;

impl Generator for Rengar {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(W, (1, 0, _1));
        ability!(E, (0, 0, _1));
    }
}