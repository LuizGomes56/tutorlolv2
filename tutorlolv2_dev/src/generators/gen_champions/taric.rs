use super::*;

impl Generator for Taric {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */, (1, _2) /* Innate [1] */])
            .ability(Key::E, [(0, _1) /* Magic Damage */])
            .end()
    }
}
