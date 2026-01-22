use super::*;

impl Generator<Champion> for Diana {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(0, 0, _1), (0, 2, _2)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(1, 0, _1), (1, 1, _2), (1, 2, _3)]);
        self.end()
    }
}
