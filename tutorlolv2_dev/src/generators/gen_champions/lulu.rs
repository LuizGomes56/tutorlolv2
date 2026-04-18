use super::*;

impl Generator for Lulu {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, 0, _1),
                (0, 1, _2),
                (1, 0, _3),
                (1, 1, _4),
                (1, 2, _5),
                (1, 3, _6),
            ],
        )
        .ability(Key::E, [(1, 0, _1)])
            .end()
    }
}
