use super::*;

impl Generator<Champion> for Evelynn {
    #[champion_generator]
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        ability![
            Q,
            (1, 0, _1),
            (2, 0, _2),
            (2, 1, _3),
            (5, 0, _4),
            (5, 1, _5),
            (5, 2, _6)
        ];
        ability![W, (2, 0, _1)];
        ability![E, (0, 0, _1), (0, 0, _2)];
        ability![R, (0, 0, _1), (1, 0, _2)];
    }
}
