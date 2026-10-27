use super::*;

impl Generator for Xerath {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Max), /* Increased Damage */
                    (2, Min), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, Void), /* Increased Damage per Stack */
                    (2, Min),  /* Magic Damage */
                    (5, Max),  /* Total Magic Damage */
                ],
            )
            .combo([Ability(E(Void)), Ability(W(Max)), Ability(Q(Void)), Attack])?
            .combo([
                Ability(Q(Void)),
                Ability(W(Max)),
                Ability(E(Void)),
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(R(Void)),
            ])?
            .combo([
                Ability(Q(Void)),
                Ability(E(Void)),
                Ability(W(Max)),
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(R(Void)),
            ])?
            .combo([Ability(E(Void)), Ability(W(Max)), Ability(Q(Void))])?
            .combo([Ability(W(Max)), Ability(Q(Void))])?
            .combo([Ability(W(Max)), Ability(E(Void)), Ability(Q(Void))])?
            .end()
    }
}
