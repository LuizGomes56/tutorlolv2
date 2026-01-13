use super::*;

impl Generator<Champion> for Rammus {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.ability(R, [(0, 0, _1)]);
        self.end()
    }
}
