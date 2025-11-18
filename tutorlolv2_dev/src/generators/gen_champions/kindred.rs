use super::*;

impl Generator<Champion> for Kindred {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(3, 0, _1), (3, 1, _2)]);
        self.ability(E, [(2, 0, _1), (2, 1, _2)]);
        self.end()
    }
}
