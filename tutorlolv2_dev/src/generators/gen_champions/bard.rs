use super::*;

// #![stable]

impl Generator<Champion> for Bard {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 1, Void)]);
        self.end()
    }
}
