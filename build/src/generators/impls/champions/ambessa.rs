use super::*;

impl Generator for Ambessa {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(1, Void) /* Medarda Maxim */])
            .damage_type(P(Void), Physical)?
            .ability_nth(
                1,
                Key::Q,
                [
                    (1, Min), /* Physical Damage */
                    (0, Max), /* Increased Physical Damage */
                ],
            )
            .ability(
                Key::Q,
                [
                    (1, _1Min), /* Physical Damage */
                    (0, _1Max), /* Increased Physical Damage */
                ],
            )
            .ability(
                Key::W,
                [
                    (1, Min), /* Physical Damage */
                    (0, Max), /* Increased Physical Damage */
                ],
            )
            .ability(
                Key::E,
                [
                    (0, Min), /* Physical Damage */
                    (1, Max), /* Total Physical Damage */
                ],
            )
            .ability(Key::R, [(2, Void) /* Physical Damage */])
            .combos([
                &[
                    Ability(Q(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(W(Max)),
                ],
            ])?
            .end()
    }
}
