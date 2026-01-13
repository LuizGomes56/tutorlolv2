use super::*;

impl Generator<Champion> for Skarner {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(
            Q,
            [
                (0, 1, _1),
                (0, 2, _2),
                (3, 0, _3),
                (0, 0, _4),
                (0, 1, _5),
            ],
        );
        self.ability(W, [(0, 0, _1)]);
        self.ability(E, [(1, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
