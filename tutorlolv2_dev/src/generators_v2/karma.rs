use super::*;

impl Generator for Karma {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(
            Q,
            (0, 0, _1),
            (0, 0, _2),
            (0, 1, _3),
            (1, 0, _4),
            (1, 1, _5),
            (1, 2, _6)
        );
        ability!(W, (0, 0, _1), (1, 0, _2));
    }
}