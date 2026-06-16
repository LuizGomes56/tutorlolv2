use super::*;

impl Generator for Zoe {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate */])
            .ability(
                Key::Q,
                [
                    (0, Max), /* Maximum Magic Damage */
                    (1, Min), /* Minimum Magic Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (2, Min), /* Magic Damage Per Bolt */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(2, Min) /* Magic Damage */])
            .clone_with(E(Min), E(Max), |dmg| f![((dmg) * MagicMultiplier) + dmg])?
            .end()
    }
}
