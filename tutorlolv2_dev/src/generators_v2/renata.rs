use super::*;

impl Generator for Renata {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(E, (1, 0, _1));
    }
}