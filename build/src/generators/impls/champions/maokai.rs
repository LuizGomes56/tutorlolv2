use super::*;

impl Generator for Maokai {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(1, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage */
                    (4, Max), /* Total Magic Damage */
                ],
            )
            .ability(Key::R, [(1, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(Q(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
