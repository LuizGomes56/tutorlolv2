use super::*;

impl Generator<Champion> for Lucian {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(0, 0, _1)]);
        self.ability(
            R,
            [
                (0, 0, _1),
                (0, 1, _2),
                (0, 2, _3),
                (0, 3, _4),
                (0, 4, _5),
                (0, 5, _6),
            ],
        );
        self.end()
    }
}
