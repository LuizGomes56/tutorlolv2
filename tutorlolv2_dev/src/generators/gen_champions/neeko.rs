use super::*;

impl Generator<Champion> for Neeko {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1), (1, 0, _2), (2, 0, _3), (2, 1, _4)]);
        self.ability(W, [(1, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(2, 0, _1)]);
        self.end()
    }
}
