use super::*;

impl Generator<Champion> for Quinn {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Q, [(0, 0, _1)]);
        self.ability(E, [(0, 0, _1)]);
        self.end()
    }
}
