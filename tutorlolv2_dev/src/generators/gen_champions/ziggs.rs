use super::*;

impl Generator<Champion> for Ziggs {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(W, [(1, 0, _1)]);
        self.ability(E, [(1, 0, _1), (1, 1, _2), (1, 2, _3)]);
        self.ability(R, [(1, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
