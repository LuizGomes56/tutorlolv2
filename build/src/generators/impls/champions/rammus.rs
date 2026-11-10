use super::*;

impl Generator for Rammus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Bonus Armor", Void)])
            .modify(W(Void), |_| f![15 + 0.1 * Armor + 0.1 * MagicResist])?
            .ability(Key::R, [("Magic Damage", Void)])
            .end()
    }
}
