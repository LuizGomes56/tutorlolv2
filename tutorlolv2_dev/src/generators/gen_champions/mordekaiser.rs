use super::*;

impl Generator<Champion> for Mordekaiser {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1), (0, 1, _2)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.end()
    }
}
