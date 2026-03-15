use super::*;

impl Generator<Champion> for Milio {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(3, 0, _1)]);
        self.end()
    }
}
