use super::*;

impl Generator<Champion> for Kayle {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1), (2, 0, _2)]);
        self.ability(R, [(1, 0, _1)]);
        self.end()
    }
}
