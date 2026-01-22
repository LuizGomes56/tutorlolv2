use super::*;

impl Generator<Champion> for Ivern {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(2, 0, _1), (3, 0, _2)]);
        self.ability(E, [(1, 0, _1)]);
        self.end()
    }
}
