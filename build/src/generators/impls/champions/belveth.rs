use super::*;

impl Generator for Belveth {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Physical Damage */])
            .ability(Key::W, [(1, Void) /* Magic Damage */])
            .ability(
                Key::E,
                [
                    (3, Max), /* Maximum Physical Damage per hit */
                    (5, Min), /* Minimum Physical Damage per hit */
                ],
            )
            .ability(
                Key::R,
                [
                    (2, _1),   /* Bonus True Damage */
                    (7, Void), /* True Damage */
                ],
            )
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Max)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Max)),
                ],
            ])?
            .end()
    }
}
