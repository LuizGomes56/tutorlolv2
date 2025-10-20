use super::*;

impl Generator for Shen {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (1, 0, _1),
            (1, 1, _2),
            (2, 0, _3),
            (2, 1, _4),
            (3, 0, _5)
        );
        ability!(E, (1, 0, _1));
    }
}