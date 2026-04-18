use super::*;

impl Generator for Nidalee {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 0, _3),
                (0, 1, _4),
                (0, 2, _5),
                (0, 3, _6),
                (1, 0, _7),
                (1, 1, _8),
            ],
        )
        .ability(Key::W, [(0, 0, _1), (0, 1, _2), (0, 0, _3)])
        .ability(Key::E, [(0, 0, _1)])
        .end()
    }
}
