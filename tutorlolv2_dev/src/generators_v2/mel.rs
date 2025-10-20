use super::*;

impl Generator for Mel {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (0, 1, _2), (0, 3, _3), (0, 4, _4));
        ability!(W, (1, 0, _1));
        ability!(
            E,
            (0, 0, _1),
            (1, 0, _2),
            (1, 1, _3),
            (2, 0, _4),
            (2, 1, _5),
            (2, 2, _6)
        );
        ability!(R, (0, 0, _1), (0, 1, _2), (1, 0, _3));
    }
}