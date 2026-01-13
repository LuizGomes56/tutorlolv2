use super::*;

impl Generator<Champion> for Rumble {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
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
        );
        self.ability(
            E,
            [
                (0, 0, _1),
                (0, 4, _2),
                (1, 0, _3),
                (1, 3, _4),
            ],
        );
        self.ability(
            R,
            [
                (1, 0, _1),
                (1, 1, _2),
                (1, 2, _3),
            ],
        );
        self.end()
    }
}
