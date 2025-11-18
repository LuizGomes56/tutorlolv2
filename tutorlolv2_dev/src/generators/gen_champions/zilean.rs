use super::*;

impl Generator<Champion> for Zilean {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(1, 0, _1)]);
        self.end()
    }
}
