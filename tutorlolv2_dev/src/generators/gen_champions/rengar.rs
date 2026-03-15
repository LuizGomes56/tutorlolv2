use super::*;

impl Generator<Champion> for Rengar {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, _1)]);
        self.ability(Key::W, [(0, 0, _1)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.end()
    }
}
