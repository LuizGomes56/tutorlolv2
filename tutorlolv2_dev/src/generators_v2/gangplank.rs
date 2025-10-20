use super::*;

impl Generator for Gangplank {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(Q, (0, 0, _1));
        ability!(E, (3, 1, _1));
        ability!(
            R,
            (0, 0, _1),
            (0, 1, _2),
            (0, 2, _3),
            (2, 0, _4),
            (2, 1, _5),
            (3, 0, _6),
            (3, 1, _7)
        );
    }
}