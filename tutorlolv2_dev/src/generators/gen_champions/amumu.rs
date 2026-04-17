use super::*;

impl Generator<Champion> for Amumu {
    fn generate(mut self: Box<Self>) -> MayFail<Champion> {
        self.ability(Key::Q, [(0, 0, Void)])
            .ability(Key::W, [(0, 0, Void)])
            .ability(Key::E, [(0, 0, Void)])
            .ability(Key::R, [(0, 0, Void)])
            .progress(Preserve);

        self.end()
    }
}
