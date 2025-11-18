use super::*;

impl Generator<Champion> for Velkoz {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(0, 0, _1), (1, 0, _2), (1, 1, _3)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(3, 0, _1), (3, 1, _2)]);
        self.end()
    }
}
