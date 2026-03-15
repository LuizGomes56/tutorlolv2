use super::*;

impl Generator<Champion> for Fiora {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(2, 0, _1)]);
        self.ability(Key::W, [(1, 0, _1)]);
        self.ability(Key::E, [(2, 0, _1)]);
        self.end()
    }
}
