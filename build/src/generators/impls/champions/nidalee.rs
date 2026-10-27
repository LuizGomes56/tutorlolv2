use super::*;

impl Generator for Nidalee {
    fn generate(&mut self) -> MayFail {
        self.ability_nth(
            1,
            Key::Q,
            [
                (1, Min), /* Minimum Magic Damage */
                (0, Max), /* Maximum Magic Damage */
            ],
        )
        .ability(
            Key::Q,
            [
                (3, _1Min), /* Minimum Magic Damage */
                (2, _1Max), /* Maximum Magic Damage */
            ],
        )
        .ability_nth(
            1,
            Key::W,
            [
                (1, Void), /* Total Magic Damage */
                (0, _1),   /* Magic Damage Per Tick */
            ],
        )
        .ability(Key::W, [(0, _2) /* Magic Damage */])
        .ability(Key::E, [(0, Void) /* Magic Damage */])
        .combos([
            &[
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Attack,
                Attack,
            ],
            &[Attack, Ability(E(Void)), Ability(Q(Max)), Ability(W(Void))],
            &[Ability(W(Void)), Ability(Q(Max)), Attack],
            &[Ability(E(Void)), Ability(W(Void)), Ability(Q(Max)), Attack],
            &[
                Ability(Q(Min)),
                Ability(W(Void)),
                Ability(E(Void)),
                Ability(Q(Min)),
                Ability(E(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Attack,
                Ability(E(Void)),
                Attack,
                Ability(W(Void)),
                Attack,
                Ability(E(Void)),
                Ability(Q(Min)),
            ],
            &[
                Ability(W(Void)),
                Ability(Q(Max)),
                Ability(E(Void)),
                Attack,
                Ability(E(Void)),
                Attack,
            ],
        ])?
        .end()
    }
}
