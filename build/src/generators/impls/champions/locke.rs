use super::*;

impl Generator for Locke {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Min) /* Innate */])
            .modify(P(Min), |_| {
                f![5 + 35 / 17 * (Level - 1) + 0.1 * AbilityPower]
            })?
            .comment(P(Min), "Silver Stake on-hit damage")?
            .clone_with(P(Min), P(Max), |p| f![2 * (p)])?
            .comment(
                P(Max),
                "Silver Stake on-hit damage against low health targets",
            )?
            .ability(
                Key::Q,
                [
                    (0, Min), /* Magic Damage per Nail */
                    (2, Max), /* Maximum Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (1, Min), /* Dash Magic Damage */
                    (2, Max), /* Total Magic Damage */
                    (0, _1),  /* Blink Magic Damage */
                ],
            )
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .damage_type(R(Void), Magic)?
            .end()
    }
}
