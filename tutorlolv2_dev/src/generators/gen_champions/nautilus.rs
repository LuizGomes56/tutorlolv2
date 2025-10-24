use super::*;

impl Generator<Champion> for Nautilus {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![Q, (1, 0, _1)];
        ability![W, (1, 0, _1)];
        ability![
            E,
            (0, 0, _1),
            (0, 1, _2),
            (0, 2, _3),
            (1, 0, _4),
            (1, 1, _5),
            (1, 2, _6)
        ];
        ability![R, (0, 0, _1), (1, 0, _2)];
    }
}
