use super::*;

impl Generator for Skarner {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (0, 1, _1),
            (0, 2, _2),
            (3, 0, _3),
            (0, 0, _4),
            (0, 1, _5)
        ];
        ability![W, (0, 0, _1)];
        ability![E, (1, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
