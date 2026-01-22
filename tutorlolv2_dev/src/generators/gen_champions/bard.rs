use super::*;

impl Generator<Champion> for Bard {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 1, _1)]);
        self.end()
    }
}
