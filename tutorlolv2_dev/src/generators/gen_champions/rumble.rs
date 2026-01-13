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
                (0, 4, _5),
                (0, 5, _6),
                (0, 6, _7),
                (0, 7, _8),
                (1, 0, _1Min),
                (1, 1, _2Min),
                (1, 2, _3Min),
                (1, 3, _4Min),
                (1, 4, _5Min),
                (1, 5, _6Min),
                (1, 6, _7Min),
                (1, 7, _8Min),
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
