use super::*;

impl Generator<Champion> for Smolder {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (0, 0, _1),
            (0, 1, _2),
            (0, 2, _3),
            (3, 0, _4),
            (3, 1, _5),
            (3, 2, _6)
        ];
        ability![W, (0, 0, _1), (0, 1, _2), (0, 2, _3)];
        ability![E, (0, 0, _1), (0, 1, _2)];
        ability![R, (0, 0, _1), (0, 1, _2)];
    }
}
