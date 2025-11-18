use super::*;

impl Generator<Champion> for Poppy {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(
            Q,
            [(0, 0, _1), (0, 1, _2), (0, 2, _3), (1, 1, _4), (1, 2, _5)],
        );
        self.ability(W, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1), (1, 1, _2)]);
        self.ability(R, [(1, 0, _1), (3, 0, _2)]);
        self.end()
    }
}
