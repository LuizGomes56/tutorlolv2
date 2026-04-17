use super::*;

impl Generator<Champion> for Sion {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (3, 0, _4),
                (3, 1, _5),
                (3, 2, _6),
                (3, 3, _7),
            ],
        )
        .ability(Key::W, [(3, 0, _1)])
        .ability(Key::E, [(0, 0, _1)])
        .ability(Key::R, [(2, 0, _1), (2, 1, _2)]);

        self.end()
    }
}
