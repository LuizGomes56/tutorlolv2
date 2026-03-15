use super::*;

impl Generator<Champion> for Blitzcrank {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::R, [(0, 0, _1), (1, 0, _2)]);
        self.end()
    }
}
