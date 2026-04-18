use super::*;

impl Generator for Varus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, 0, _1), (2, 1, _2), (2, 2, _3), (2, 3, _4)])
            .ability(
                Key::W,
                [
                    (0, 0, _1),
                    (0, 1, _2),
                    (1, 0, _3),
                    (1, 1, _4),
                    (1, 2, _5),
                    (1, 3, _6),
                    (4, 0, _7),
                ],
            )
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1)])
            .end()
    }
}
