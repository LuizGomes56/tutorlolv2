use super::*;

impl Generator for Orianna {
    fn generate(&mut self) -> MayFail {
        let pmax = self
            .ability(Key::P, [(1, Min) /* Innate */])
            .merge_damage([P(Min)], |[p]| f![1.4 * p])?;

        self.clone_to(P(Min), P(Max), pmax)?
            .ability(
                Key::Q,
                [
                    (0, Max), /* Magic Damage */
                    (1, Min), /* Reduced Damage */
                ],
            )
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(1, Void) /* Magic Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                ],
                &[Ability(Q(Max)), Attack, Ability(W(Void))],
                &[Ability(Q(Min)), Ability(Q(Min)), Ability(W(Void)), Attack],
                &[
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Max)), Ability(W(Void))],
                &[Ability(Q(Max)), Attack, Ability(W(Void)), Ability(E(Void))],
            ])?
            .end()
    }
}
