use super::*;

impl Generator for Tristana {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(W, (0, 0, _1));
        ability!(
            E,
            (0, 0, _1),
            (1, 0, _2),
            (2, 0, _3),
            (2, 1, _4),
            (2, 2, _5)
        );
        ability!(R, (0, 0, _1));
    }
}