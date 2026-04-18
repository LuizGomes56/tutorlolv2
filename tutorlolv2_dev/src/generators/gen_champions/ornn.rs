use super::*;

impl Generator for Ornn {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1)])
            .ability(
                Key::W,
                [
                    (1, 0, _1),
                    (1, 1, _2),
                    (1, 2, _3),
                    (1, 3, _4),
                    (2, 0, _5),
                    (2, 1, _6),
                ],
            )
            .ability(Key::E, [(0, 0, _1)])
            .ability(Key::R, [(0, 0, _1), (3, 0, _2)])
            .end()
    }
}
