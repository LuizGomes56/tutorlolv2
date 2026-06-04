use super::*;

impl Generator for Rammus {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void)])
            .modify(W(Void), |_| {
                15.plus(0.1).times(Armor).plus(0.1).times(MagicResist)
            })?
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .end()
    }
}
