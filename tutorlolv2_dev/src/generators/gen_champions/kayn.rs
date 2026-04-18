use super::*;

impl Generator for Kayn {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (1, 0, _3),
                (1, 1, _4),
                (2, 0, _5),
                (2, 1, _6),
            ],
        )
        .ability(Key::W, [(0, 0, _1)])
        .ability(Key::R, [(3, 0, _1)])
        .end()
    }
}
