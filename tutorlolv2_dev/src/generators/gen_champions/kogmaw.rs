use super::*;

impl Generator<Champion> for KogMaw {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(W, [(0, 1, _1)]);
        self.ability(E, [(1, 0, _1)]);
        self.ability(R, [(0, 0, _1), (0, 1, _2)]);
        self.end()
    }
}
