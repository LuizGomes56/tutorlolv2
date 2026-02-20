use super::*;

impl Generator<Champion> for Senna {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(0, 0, _1)]);
        self.ability(Key::R, [(2, 0, _1)]);
        self.end()
    }
}
