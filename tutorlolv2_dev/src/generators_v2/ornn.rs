use super::*;

impl Generator for Ornn {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(
            W,
            (1, 0, _1),
            (1, 1, _2),
            (2, 0, _3),
            (2, 1, _4),
            (2, 2, _5),
            (2, 3, _6)
        );
        ability!(E, (0, 0, _1));
        ability!(R, (0, 0, _1), (2, 0, _2));
    }
}