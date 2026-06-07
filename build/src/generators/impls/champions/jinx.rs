use super::*;

impl Generator for Jinx {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void)])
            .modify(Q(Void), |_| 1.1.times(AttackDamage))?
            .ability(Key::W, [(0, Void) /* Physical Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Max),   /* Maximum Physical Damage */
                    (1, _1Max), /* Maximum Secondary Damage */
                    (2, Min),   /* Minimum Physical Damage */
                    (3, _1Min), /* Minimum Secondary Damage */
                ],
            )
            .end()
    }
}
