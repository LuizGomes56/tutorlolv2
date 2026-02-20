use super::*;

impl Generator<Champion> for Rammus {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(1, 0, _1)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.ability(Key::R, [(0, 0, _1)]);
        self.end()
    }
}
