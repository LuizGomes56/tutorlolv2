use super::*;

impl Generator for Malzahar {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .ability(
                Key::R,
                [
                    (0, Min), /* Magic Damage Per Tick */
                    (1, Max), /* Total Magic Damage */
                ],
            )
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(E(Max)), Ability(Q(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
            ])?
            .end()
    }
}
