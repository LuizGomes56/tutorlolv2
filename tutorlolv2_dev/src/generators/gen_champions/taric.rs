use super::*;

impl Generator<Champion> for Taric {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(E, [(0, 0, _1)]);
        self.end()
    }
}
