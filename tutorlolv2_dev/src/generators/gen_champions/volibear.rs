use super::*;

impl Generator for Volibear {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Lightning Claws */])
            .ability(Key::Q, [(1, Void) /* Bonus Physical Damage */])
            .ability(Key::W, [(2, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(Key::R, [(1, Void) /* Physical Damage */])
            .end()
    }
}
