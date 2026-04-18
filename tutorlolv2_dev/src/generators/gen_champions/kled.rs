use super::*;

impl Generator for Kled {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (2, 0, _3),
                (2, 2, _4),
                (0, 0, _5),
                (2, 0, _6),
                (2, 1, _7),
            ],
        )
        .ability(Key::W, [(0, 0, _1), (2, 0, _2)])
        .ability(Key::E, [(0, 0, _1), (2, 0, _2)])
        .ability(Key::R, [(1, 0, _1), (1, 1, _2)])
        .end()
    }
}
