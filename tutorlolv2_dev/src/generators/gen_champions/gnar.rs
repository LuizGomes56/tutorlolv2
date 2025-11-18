use super::*;

impl Generator<Champion> for Gnar {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 1, _2), (0, 0, _3)]);
        self.ability(W, [(2, 0, _1), (0, 0, _2)]);
        self.ability(E, [(4, 0, _1), (0, 0, _2)]);
        self.ability(R, [(0, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
