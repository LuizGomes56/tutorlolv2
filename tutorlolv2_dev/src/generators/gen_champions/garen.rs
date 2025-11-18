use super::*;

impl Generator<Champion> for Garen {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(W, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1), (3, 0, _2)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
