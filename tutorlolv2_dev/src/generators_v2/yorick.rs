use super::*;

impl Generator for Yorick {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(Q, (0, 0, _1));
        ability!(E, (0, 0, _1), (0, 1, _2), (0, 2, _3));
    }
}
