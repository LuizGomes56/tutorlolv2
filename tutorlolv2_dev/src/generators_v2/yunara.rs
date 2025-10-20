use super::*;

impl Generator for Yunara {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(
            Q,
            (0, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (1, 3, _4),
            (1, 4, _5)
        );
        ability!(W, (0, 0, _1), (0, 1, _2), (0, 2, _3));
        ability!(R, (0, 0, _1));
    }
}