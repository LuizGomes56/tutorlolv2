use super::*;

impl Generator for Teemo {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 1, _1)];
        ability![
            E,
            (0, 0, _1),
            (0, 1, _2),
            (0, 2, _3),
            (1, 0, _4),
            (1, 1, _5),
            (1, 2, _6)
        ];
        ability![R, (5, 0, _1), (5, 1, _2)];
    }
}
