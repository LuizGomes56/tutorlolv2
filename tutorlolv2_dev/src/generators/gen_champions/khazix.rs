use super::*;

impl Generator<Champion> for Khazix {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (0, 1, _2)]);
        self.ability(W, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.end()
    }
}
