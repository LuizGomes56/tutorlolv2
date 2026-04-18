use super::*;

impl Generator for Pantheon {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (4, 0, _1),
                (4, 1, _2),
                (4, 2, _3),
                (4, 3, _4),
                (5, 0, _5),
                (5, 1, _6),
            ],
        )
        .ability(Key::W, [(0, 0, _1)])
        .ability(Key::E, [(4, 0, _1)])
        .ability(Key::R, [(3, 0, _1), (3, 1, _2)])
        .end()
    }
}
