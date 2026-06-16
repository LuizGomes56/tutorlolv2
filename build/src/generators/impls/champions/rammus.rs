use super::*;

impl Generator for Rammus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void)])
            .modify(W(Void), |_| f![15 + 0.1 * Armor + 0.1 * MagicResist])?
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
