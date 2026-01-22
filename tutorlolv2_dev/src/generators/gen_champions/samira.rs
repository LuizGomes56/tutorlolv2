use super::*;

impl Generator<Champion> for Samira {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(2, 0, _1), (2, 1, _2)]);
        self.ability(E, [(0, 1, _1)]);
        self.ability(R, [(1, 0, _1), (1, 1, _2), (1, 2, _3), (1, 3, _4)]);
        self.end()
    }
}
