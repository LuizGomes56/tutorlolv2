use super::*;

impl Generator<Champion> for Tryndamere {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(1, 1, _1)]);
        self.ability(W, [(1, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.end()
    }
}
