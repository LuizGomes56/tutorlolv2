use super::*;

impl Generator for Janna {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .ability(
                Key::Q,
                [
                    (0, _1),  /* Bonus Damage Per Second */
                    (1, Max), /* Maximum Magic Damage */
                    (2, Min), /* Minimum Magic Damage */
                ],
            )
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .damage_type(P(Void), Magic)?
            .combos([
                &[Ability(Q(Min)), Ability(Q(Min)), Ability(W(Void))],
                &[Ability(Q(Max)), Ability(W(Void))],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Min)), Ability(Q(Min)), Attack],
            ])?
            .end()
    }
}
