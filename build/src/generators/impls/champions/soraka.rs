use super::*;

impl Generator for Soraka {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(2, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (2, Max), /* Total Magic Damage */
                ],
            )
            .combos([
                &[Ability(Q(Void)), Ability(E(Max)), Attack, Attack],
                &[Ability(E(Max)), Ability(Q(Void))],
                &[Ability(E(Max)), Ability(Q(Void)), Attack, Attack],
                &[Ability(Q(Void)), Attack, Ability(E(Max))],
            ])?
            .end()
    }
}
