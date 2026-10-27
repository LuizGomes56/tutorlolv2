use super::*;

impl Generator for Renata {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(2, Void) /* Innate */])
            .modify(P(Void), |_| {
                f![(0.01 + 0.01 / 17 * (Level - 1) + 0.0002 * AbilityPower) * EnemyMaxHealth]
            })?
            .ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(0, Void) /* Magic Damage */])
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(Q(Void)), Ability(E(Void))],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(Q(Void)), Attack],
                &[
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(Q(Void)), Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
