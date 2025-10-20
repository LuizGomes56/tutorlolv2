use super::*;

impl Generator for Aphelios {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(P, (3, 0, _1));
    }
}