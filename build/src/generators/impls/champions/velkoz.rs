use super::*;

impl Generator for Velkoz {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, Min), /* Magic Damage */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (0, Min), /* Damage Per Tick */
                    (1, Max), /* Maximum Damage */
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(R(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(W(Min)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[
                    Ability(E(Void)),
                    Ability(W(Min)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Ability(E(Void)),
                    Ability(W(Min)),
                ],
                &[Ability(E(Void)), Ability(W(Max))],
            ])?
            .end()
    }
}
