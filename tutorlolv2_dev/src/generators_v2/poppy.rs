use super::*;

impl Generator for Poppy {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (0, 2, _3),
            (1, 1, _4),
            (1, 2, _5)
        ];
        ability![W, (0, 0, _1)];
        ability![E, (0, 0, _1), (1, 1, _2)];
        ability![R, (1, 0, _1), (3, 0, _2)];
    }
}
