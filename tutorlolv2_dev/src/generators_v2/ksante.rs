use super::*;

impl Generator for KSante {
    #[generator_v2]
    fn generate(self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![
            W,
            (1, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (4, 0, _4),
            (4, 1, _5)
        ];
        ability![R, (0, 0, _1), (3, 0, _2), (3, 1, _3)];
    }
}
