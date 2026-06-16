use super::*;

impl Generator for Renata {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate */])
            .modify(P(Void), |dmg| f![dmg * EnemyMaxHealth])?
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .end()
    }
}
