use super::*;

impl Generator<Champion> for Mel {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 1, _2), (0, 3, _3), (0, 4, _4)]);
        self.ability(W, [(1, 0, _1)]);
        self.ability(
            E,
            [
                (0, 0, _1),
                (1, 0, _2),
                (1, 1, _3),
                (1, 2, _4),
                (2, 0, _5),
                (2, 1, _6),
            ],
        );
        self.ability(R, [(0, 0, _1), (2, 0, _2), (2, 1, _3)]);
        self.end()
    }
}
