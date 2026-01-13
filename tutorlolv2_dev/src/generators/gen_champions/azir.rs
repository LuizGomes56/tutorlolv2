use super::*;

impl Generator<Champion> for Azir {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(W, [(3, 0, _1)]);
        self.ability(E, [(1, 0, _1)]);
        self.ability(R, [(1, 0, _1)]);
        self.end()
    }
}
