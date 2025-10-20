use super::*;

impl Generator for Pantheon {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(
            Q,
            (1, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (1, 3, _4),
            (2, 0, _5),
            (2, 1, _6)
        );
        ability!(W, (0, 0, _1));
        ability!(E, (2, 0, _1));
        ability!(R, (3, 0, _1), (3, 1, _2));
    }
}