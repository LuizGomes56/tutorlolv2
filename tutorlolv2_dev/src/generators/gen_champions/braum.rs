use super::*;

impl Generator<Champion> for Braum {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::E, [(0, 1, _1)]);
        self.ability(Key::R, [(1, 0, _1)]);
        self.end()
    }
}
