use super::*;

impl Generator<Champion> for Evelynn {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (1, 0, _1),
                (2, 0, _2),
                (2, 1, _3),
                (5, 0, _4),
                (5, 1, _5),
                (5, 2, _6),
            ],
        );
        self.ability(W, [(2, 0, _1)]);
        self.ability(
            E,
            [(0, 0, _1), (0, 0, _2)],
        );
        self.ability(
            R,
            [(0, 0, _1), (1, 0, _2)],
        );
        self.end()
    }
}
