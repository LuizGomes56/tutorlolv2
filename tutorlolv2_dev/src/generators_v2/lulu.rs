use super::*;

impl Generator for Lulu {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (1, 0, _3),
            (1, 1, _4),
            (1, 2, _5),
            (1, 3, _6)
        ];
        ability![E, (1, 0, _1)];
    }
}
