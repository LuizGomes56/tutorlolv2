use super::*;

impl Generator<Champion> for Kaisa {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1), (2, 0, _2), (3, 0, _3), (3, 1, _4)]);
        self.ability(W, [(0, 0, _1)]);
        self.end()
    }
}
