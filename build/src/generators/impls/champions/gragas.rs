use super::*;

impl Generator for Gragas {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Maximum Magic Damage */
                (3, Min), /* Minimum Magic Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Bonus Magic Damage */])
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .ability(Key::R, [(0, Void) /* Magic Damage */])
        .combos([
            &[
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(R(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Ability(Q(Min)),
                Ability(R(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
            ],
            &[Ability(E(Void)), Ability(R(Void))],
            &[
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Ability(Q(Min)),
            ],
            &[Ability(Q(Max)), Ability(W(Void))],
        ])?
        .end()
    }
}
