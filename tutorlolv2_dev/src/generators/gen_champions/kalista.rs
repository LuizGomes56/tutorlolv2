use super::*;

impl Generator<Champion> for Kalista {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(1, 0, _1), (1, 1, _2)]);
        self.ability(E, [(1, 0, _1), (1, 1, _2)]);
        self.end()
    }
}
