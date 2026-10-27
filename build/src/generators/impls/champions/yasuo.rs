use super::*;

impl Generator for Yasuo {
    fn generate(&mut self) -> MayFail {
        self.ability(
            Key::Q,
            [
                (0, Max), /* Critical Strike Damage */
                (1, Min), /* Physical Damage */
            ],
        )
        .ability(
            Key::E,
            [
                (0, _1Min), /* Bonus Damage per Stack */
                (1, Min),   /* Magic Damage */
                (2, _1Max), /* Maximum Bonus Damage */
                (3, Max),   /* Total Combined Damage */
            ],
        )
        .ability(Key::R, [(0, Void) /* Physical Damage */])
        .combos([
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[
                Ability(E(Min)),
                Ability(Q(Min)),
                Ability(E(Min)),
                Ability(Q(Min)),
                Ability(R(Void)),
                Attack,
                Ability(Q(Min)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(E(Max)),
                Ability(Q(Min)),
                Attack,
                Ability(R(Void)),
                Attack,
            ],
            &[
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(Q(Min)),
                Ability(R(Void)),
            ],
            &[Ability(E(Min)), Attack, Ability(Q(Max)), Ability(E(Min))],
        ])?
        .end()
    }
}
