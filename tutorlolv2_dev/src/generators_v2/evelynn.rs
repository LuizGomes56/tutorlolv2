use super::*;

impl Generator for Evelynn {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(
            Q,
            (0, 0, _1),
            (1, 0, _2),
            (1, 1, _3),
            (3, 0, _4),
            (3, 1, _5),
            (3, 2, _6)
        );
        ability!(W, (4, 0, _1));
        ability!(E, (0, 0, _1), (0, 0, _2));
        ability!(R, (0, 0, _1), (1, 0, _2));
    }
}