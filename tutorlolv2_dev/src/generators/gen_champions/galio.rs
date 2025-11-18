use super::*;

impl Generator<Champion> for Galio {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(0, 0, _1), (0, 1, _2), (2, 0, _3), (2, 1, _4)]);
        self.ability(E, [(1, 0, _1), (1, 1, _2)]);
        self.ability(R, [(1, 0, _1)]);
        self.end()
    }
}
