use super::*;

impl Generator for Pantheon {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (4, 0, _1),
            (4, 1, _2),
            (4, 2, _3),
            (4, 3, _4),
            (5, 0, _5),
            (5, 1, _6)
        ];
        ability![W, (0, 0, _1)];
        ability![E, (4, 0, _1)];
        ability![R, (3, 0, _1), (3, 1, _2)];
    }
}
