use super::*;

impl Generator for MonkeyKing {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, Void) /* Bonus Physical Damage */])
            .ability(Key::W, [(0, Void) /* Clone Outgoing Damage */])
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, _1Max), /* Maximum Total Physical Damage (Recast) */
                    (1, Min),   /* Physical Damage Per Tick */
                    (2, Max),   /* Total Physical Damage */
                ],
            )
            .damage_types([W(Void), R(Min), R(Max), R(_1Max)], Physical)?
            .modify(W(Void), |ratio| ratio.times(AttackDamage))?
            .end()
    }
}
