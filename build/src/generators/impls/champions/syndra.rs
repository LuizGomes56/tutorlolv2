use super::*;

impl Generator for Syndra {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Void), /* Bonus Damage */
                    (1, Min),  /* Magic Damage */
                    (2, Max),  /* Total Mixed Damage */
                ],
            );

        let w_bonus = self.damage_of(W(Void))?.clone();

        self.modify(W(Max), |dmg: &str| f![((dmg) * MagicMultiplier) + w_bonus])?
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, Void), /* Magic Damage per Sphere */
                    (2, Max),  /* Maximum Magic Damage */
                    (3, Min),  /* Minimum Magic Damage */
                ],
            )
            .end()
    }
}
