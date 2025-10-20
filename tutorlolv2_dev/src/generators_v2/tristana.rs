use super::*;

impl Generator for Tristana {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![W, (0, 0, _1)];
        ability![
            E,
            (0, 0, _1),
            (2, 0, _2),
            (3, 0, _3),
            (3, 1, _4),
            (3, 2, _5)
        ];
        ability![R, (0, 1, _1)];
    }
}
