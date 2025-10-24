use super::*;

impl Generator<Champion> for Ornn {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (0, 0, _1)];
        ability![
            W,
            (1, 0, _1),
            (1, 1, _2),
            (1, 2, _3),
            (1, 3, _4),
            (2, 0, _5),
            (2, 1, _6)
        ];
        ability![E, (0, 0, _1)];
        ability![R, (0, 0, _1), (3, 0, _2)];
    }
}
