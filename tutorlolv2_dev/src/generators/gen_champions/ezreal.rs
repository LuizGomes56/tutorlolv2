use super::*;

// #![stable]

impl Generator<Champion> for Ezreal {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, Void)]);
        self.ability(Key::W, [(1, 0, Void)]);
        self.ability(Key::E, [(0, 0, Void)]);
        self.ability(Key::R, [(0, 0, Void), /* (1, 0, Minion) */]);
        self.end()
    }
}
