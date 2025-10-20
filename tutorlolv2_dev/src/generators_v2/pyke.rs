use super::*;

impl Generator for Pyke {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (1, 0, _1));
        ability!(E, (1, 0, _1));
    }
}