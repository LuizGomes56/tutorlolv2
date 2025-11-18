use super::*;

impl Generator<Champion> for Urgot {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(1, 0, _1)]);
        self.ability(E, [(1, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
