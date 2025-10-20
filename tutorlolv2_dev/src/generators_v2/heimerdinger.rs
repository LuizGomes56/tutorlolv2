use super::*;

impl Generator for Heimerdinger {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![
            W,
            (0, 0, _1),
            (1, 0, _2),
            (1, 1, _3),
            (1, 2, _4),
            (1, 3, _5),
            (1, 4, _6),
            (1, 5, _7),
            (0, 0, _8),
            (0, 1, _1Min),
            (0, 2, _2Min),
            (0, 3, _3Min),
            (0, 4, _4Min)
        ];
        ability![E, (0, 0, _1)];
    }
}
