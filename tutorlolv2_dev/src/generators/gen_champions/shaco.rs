use super::*;

impl Generator<Champion> for Shaco {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![
            W,
            (1, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (1, 3, _4),
            (1, 4, _5)
        ];
        ability![E, (0, 0, _1), (0, 1, _2)];
        ability![R, (3, 0, _1), (3, 1, _2), (4, 0, _3)];
    }
}
