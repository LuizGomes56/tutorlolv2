use super::*;

impl Generator<Champion> for Ambessa {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 1, _2), (0, 0, _3), (0, 1, _4)]);
        self.ability(W, [(0, 0, _1), (1, 0, _2)]);
        self.ability(E, [(0, 0, _1), (0, 1, _2)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
