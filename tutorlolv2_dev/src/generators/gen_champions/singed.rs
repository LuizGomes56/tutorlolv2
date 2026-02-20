use super::*;

impl Generator<Champion> for Singed {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(2, 0, _1), (2, 1, _2), (2, 2, _3)]);
        self.ability(Key::E, [(0, 0, _1)]);
        self.end()
    }
}
