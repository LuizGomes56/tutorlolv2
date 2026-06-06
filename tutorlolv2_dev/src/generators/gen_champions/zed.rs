use super::*;

impl Generator for Zed {
    fn generate(&mut self) -> MayFail {
        let p1 = self.formula(Key::P, 3)?.to_string();

        self.ability(Key::P, [(2, Void) /* Innate */])
            .modify(P(Void), |dmg| dmg.replace(&p1, "").times(EnemyMaxHealth))?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Physical Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Physical Damage */])
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .end()
    }
}
