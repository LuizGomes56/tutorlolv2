use super::*;

impl Generator<Champion> for Volibear {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(W, [(0, 0, _1)]);
        self.ability(E, [(1, 0, _1)]);
        self.ability(R, [(4, 0, _1)]);
        self.end()
    }
}
