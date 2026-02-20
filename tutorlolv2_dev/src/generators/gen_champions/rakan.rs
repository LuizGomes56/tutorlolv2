use super::*;

impl Generator<Champion> for Rakan {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(0, 0, _1)]);
        self.ability(Key::R, [(0, 1, _1)]);
        self.end()
    }
}
