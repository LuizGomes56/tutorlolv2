use super::*;

impl Generator for Blitzcrank {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(R, (0, 0, _1), (1, 0, _2));
    }
}