use super::*;

impl Generator for Kennen {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(
                Key::W,
                [
                    (0, _1),   /* Bonus Magic Damage */
                    (1, Void), /* Magic Damage */
                ],
            )
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(
                Key::R,
                [
                    (1, Min), /* Magic Damage Per Bolt */
                    (2, Max), /* Total Single-Target Damage */
                ],
            )
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(W(Void))],
                &[Attack, Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
