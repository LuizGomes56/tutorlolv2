use super::*;

impl Generator for Milio {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, _1) /* Innate */, (1, _2) /* Innate [1] */])
            .ability(Key::Q, [(0, _1) /* Magic Damage */])
            .end()
    }
}
