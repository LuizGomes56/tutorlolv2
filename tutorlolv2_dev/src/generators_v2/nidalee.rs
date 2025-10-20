use super::*;

impl Generator for Nidalee {
    #[generator_v2]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (0, 0, _3),
            (0, 1, _4),
            (0, 2, _5),
            (0, 3, _6),
            (1, 0, _7),
            (1, 1, _8)
        ];
        ability![W, (0, 0, _1), (0, 1, _2), (0, 0, _3)];
        ability![E, (0, 0, _1)];
    }
}
