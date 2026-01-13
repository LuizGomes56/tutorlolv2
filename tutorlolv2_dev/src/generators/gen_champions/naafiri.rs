use super::*;

impl Generator<Champion> for Naafiri {
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
                (1, 1, _5),
                (1, 2, _6),
                (1, 3, _7),
                (1, 4, _8),
            ],
        );
        self.ability(
            E,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
            ],
        );
        self.ability(
            R,
            [(0, 0, _1), (0, 1, _2)],
        );
        self.end()
    }
}
