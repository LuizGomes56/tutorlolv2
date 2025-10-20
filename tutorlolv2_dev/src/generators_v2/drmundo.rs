use super::*;

impl Generator for DrMundo {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1), (1, 0, _2), (1, 1, _3));
        ability!(W, (0, 0, _1), (0, 1, _2), (2, 0, _3));
        ability!(
            E,
            (0, 0, _1),
            (1, 0, _2),
            (1, 1, _3),
            (2, 0, _4),
            (2, 1, _5),
            (2, 2, _6),
            (2, 3, _7)
        );
    }
}