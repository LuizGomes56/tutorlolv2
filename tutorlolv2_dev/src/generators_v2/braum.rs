use super::*;

impl Generator for Braum {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1));
    }
}