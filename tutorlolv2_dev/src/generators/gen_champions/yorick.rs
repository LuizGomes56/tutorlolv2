use super::*;

impl Generator for Yorick {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, _1) /* Innate - Final Service */])
            .ability(Key::Q, [(0, _1) /* Bonus Physical Damage */])
            .ability(Key::E, [(3, _1) /* Magic Damage */])
            .end()
    }
}
