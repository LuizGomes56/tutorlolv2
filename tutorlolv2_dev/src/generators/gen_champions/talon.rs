use super::*;

impl Generator<Champion> for Talon {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 1, _2)]);
        self.ability(W, [(0, 0, _1), (1, 0, _2), (1, 2, _3)]);
        self.ability(R, [(0, 1, _1), (1, 0, _2)]);
        self.end()
    }
}
