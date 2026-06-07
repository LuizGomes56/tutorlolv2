use super::*;

impl Generator for Hecarim {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, Void) /* Physical Damage */])
            .modify(Q(Void), |dmg| {
                /* Rampage bonus damage per stack */
                dmg.parenthesize().times(
                    1.03.plus(0.03)
                        .times(BonusAd)
                        .div(100)
                        .parenthesize()
                        .times(Stacks),
                )
            })?
            .ability(
                Key::W,
                [
                    (2, Min), /* Magic Damage Per Tick */
                    (3, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Max), /* Maximum Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .ability(Key::R, [(0, Void) /* Magic damage */])
            .end()
    }
}
