use super::*;

impl Generator for Gwen {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (0, 3, _4),
                (1, 0, _5),
                (1, 1, _6),
                (1, 2, _7),
                (1, 3, _8),
            ],
        )
        .ability(
            Key::R,
            [(0, 0, _1), (0, 1, _2), (3, 0, _3), (3, 1, _4), (3, 2, _5)],
        )
        .end()
    }
}
