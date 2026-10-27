use super::*;

impl Generator for Kayn {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (2, Min), /* Physical Damage */
                (5, Max), /* Total Physical Damage */
            ],
        )
        .ability(Key::W, [(0, Void) /* Physical Damage */])
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .combos([
            &[Ability(W(Void)), Ability(Q(Max))],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(R(Void)),
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(R(Void)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Ability(R(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Attack,
                Ability(R(Void)),
                Ability(R(Void)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(R(Void)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Ability(W(Void)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Ability(W(Void)),
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
            ],
            &[
                Attack,
                Ability(Q(Min)),
                Attack,
                Ability(W(Void)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Attack,
                Ability(R(Void)),
                Ability(R(Void)),
            ],
            &[Ability(W(Void)), Ability(Q(Max)), Attack],
            &[Ability(Q(Max)), Attack, Ability(W(Void)), Attack],
        ])?
        .end()
    }
}
