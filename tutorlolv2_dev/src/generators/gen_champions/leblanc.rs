use super::*;

impl Generator for Leblanc {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, 0, _1), (1, 0, _2)])
            .ability(Key::W, [(0, 0, _1)])
            .ability(Key::E, [(0, 0, _1), (1, 0, _2), (1, 1, _3)])
            .ability(
                Key::R,
                [
                    (1, 0, _1),
                    (2, 0, _2),
                    (2, 1, _3),
                    (2, 2, _4),
                    (3, 0, _5),
                    (3, 1, _6),
                    (3, 2, _7),
                ],
            )
            .end()
    }
}
