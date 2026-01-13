use super::*;

impl Generator<Champion> for Pyke {
    fn generate(
        mut self: Box<Self>,
    ) -> MayFail<Champion> {
        self.ability(Q, [(2, 0, _1)]);
        self.ability(E, [(1, 0, _1)]);
        self.end()
    }
}
