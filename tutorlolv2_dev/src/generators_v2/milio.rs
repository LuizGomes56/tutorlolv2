use super::*;

impl Generator for Milio {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (1, 0, _1));
    }
}