use super::*;

impl Generator<Champion> for Evelynn {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Key::Q,
            [
                (1, 0, _1),
                (2, 0, _2),
                (2, 1, _3),
                (5, 0, _4),
                (5, 1, _5),
                (5, 2, _6),
            ],
        )
        .ability(Key::W, [(2, 0, _1)])
        .ability(Key::E, [(0, 0, _1), (0, 0, _2)])
        .ability(Key::R, [(0, 0, _1), (1, 0, _2)]);

        self.end()
    }
}
