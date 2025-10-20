use super::*;

impl Generator for Volibear {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (1, 0, _1));
        ability!(W, (0, 0, _1));
        ability!(E, (1, 0, _1));
        ability!(R, (2, 0, _1));
    }
}