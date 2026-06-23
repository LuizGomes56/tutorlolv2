use super::*;

impl Generator for Samira {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::P,
            [
                (0, _1Min), /* Description 3 */
                (3, Min),   /* Innate */
            ],
        )
        .clone_with(P(Min), P(Max), |dmg| f![(dmg) * 2])?;

        let p = self.scaling(Key::P, 0)?[0]
            .clone()
            .set_ctx_var(AttackDamage)
            .render(Level)?;

        self.set_damage(P(_1Min), p)?
            .comment(P(_1Min), "Passive against immobilized targets")?
            .clone_with(P(_1Min), P(_1Max), |dmg| f![(dmg) * 6])?
            .ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Physical Damage per Hit */
                    (1, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, Min), /* Physical Damage Per Shot */
                    (3, Max), /* Total Physical Damage */
                ],
            )
            .end()
    }
}
