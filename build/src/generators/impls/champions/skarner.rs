use super::*;

impl Generator for Skarner {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Innate [1] */])
            .modify(P(Void), |_| {
                f![(0.05 + 0.04 / 17 * (Level - 1)) * EnemyMaxHealth]
            })?
            .ability_nth(
                1,
                Key::Q,
                [
                    (1, Min), /* Bonus Physical Damage per Hit */
                    (3, Max), /* Total Bonus Physical Damage */
                ],
            )
            .ability(Key::Q, [(1, Void) /* Physical Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Physical Damage */])
            .ability(Key::R, [(0, Void) /* Magic Damage */])
            .damage_type(W(Void), Magic)?
            .combos([
                &[Ability(R(Void)), Ability(W(Void))],
                &[Ability(E(Void)), Ability(R(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(W(Void)), Attack, Attack],
            ])?
            .end()
    }
}
