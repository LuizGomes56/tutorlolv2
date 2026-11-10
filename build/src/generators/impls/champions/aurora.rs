use super::*;

impl Generator for Aurora {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [("Innate", Void)])
            .modify(P(Void), |_| {
                f![(0.01 + 0.00027 * AbilityPower) * EnemyMaxHealth]
            })?
            .comment(P(Void), "Third stack bonus magic damage")?
            .ability(
                Key::Q,
                [
                    ("Maximum Magic Damage", Max),
                    ("Minimum Magic Damage", Min),
                    ("Subsequent Bolt Maximum Magic Damage", _1Max),
                    ("Subsequent Bolt Minimum Magic Damage", _1Min),
                ],
            )
            .ability(Key::E, [("Magic Damage", Void)])
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Min)), Ability(E(Void)), Ability(Q(Min))],
                &[Ability(Q(Min)), Attack, Ability(E(Void)), Ability(Q(Min))],
            ])?
            .end()
    }
}
