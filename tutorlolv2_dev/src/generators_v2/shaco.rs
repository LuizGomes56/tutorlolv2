use super::*;

impl Generator for Shaco {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (2, 0, _1));
        ability!(
            W,
            (2, 0, _1),
            (2, 1, _2),
            (2, 2, _3),
            (2, 3, _4),
            (2, 4, _5)
        );
        ability!(E, (1, 0, _1), (1, 1, _2));
        ability!(R, (2, 0, _1), (3, 0, _2), (3, 1, _3));
    }
}