use super::*;

impl Generator for Ryze {
    fn generate(&mut self) -> MayFail {
        let r = &self.formula(Key::R, 0)?.to_string();

        self.ability(Key::Q, [(1, Void) /* Magic Damage */])
            .modify(Q(Void), |dmg| f![(dmg) * r])?
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .end()
    }
}
