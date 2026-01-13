use super::*;

impl Generator<Champion> for Nautilus {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(W, [(1, 0, _1)]);
        self.ability(
            E,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (1, 0, _4),
                (1, 1, _5),
                (1, 2, _6),
            ],
        );
        self.ability(
            R,
            [(0, 0, _1), (1, 0, _2)],
        );
        self.end()
    }
}
