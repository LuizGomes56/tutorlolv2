use super::*;

impl Generator for Sylas {
    #[generator_v2]
    fn generate(self: Box<Self>) -> Champion {
        ability!(
            Q,
            (0, 0, _1),
            (1, 0, _2),
            (1, 1, _3),
            (1, 2, _4),
            (1, 3, _5)
        );
        ability!(W, (0, 0, _1));
        ability!(E, (0, 0, _1));
    }
}