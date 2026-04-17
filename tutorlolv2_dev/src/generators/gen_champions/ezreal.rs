use super::*;

impl Generator<Champion> for Ezreal {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(1, 0, Void)])
            .ability(Key::E, [(0, 0, Void)])
            .ability(Key::R, [(0, 0, Void) /* (1, 0, Minion) */])
            .progress(Preserve);

        self.end()
    }
}
