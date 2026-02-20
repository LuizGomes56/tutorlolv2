use super::*;

impl Generator<Champion> for Taric {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::E, [(0, 0, _1)]);
        self.end()
    }
}
