use super::*;

impl Generator for Aurora {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability!(
            Q,
            (0, 0, _1),
            (2, 0, _2),
            (2, 1, _3),
            (2, 2, _4),
            (2, 3, _5)
        );
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1));
    }
}