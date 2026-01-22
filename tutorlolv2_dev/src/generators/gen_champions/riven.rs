use super::*;

impl Generator<Champion> for Riven {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(2, 0, _1), (2, 1, _2)]);
        self.ability(W, [(0, 0, _1)]);
        self.ability(R, [(0, 0, _1), (0, 1, _2)]);
        self.end()
    }
}
