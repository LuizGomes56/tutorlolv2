use super::*;

impl Generator for Varus {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (2, 0, _1), (2, 1, _2), (2, 2, _3), (2, 3, _4)];
        ability![
            W,
            (0, 0, _1),
            (0, 1, _2),
            (1, 0, _3),
            (1, 1, _4),
            (1, 2, _5),
            (1, 3, _6),
            (4, 0, _7)
        ];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1)];
    }
}
