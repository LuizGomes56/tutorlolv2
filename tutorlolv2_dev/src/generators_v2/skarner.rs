use super::*;

impl Generator for Skarner {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (1, 0, _3),
            (0, 0, _4),
            (0, 1, _5)
        );
        ability!(W, (0, 0, _1));
        ability!(E, (2, 0, _1));
        ability!(R, (0, 0, _1));
    }
}