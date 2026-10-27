use super::*;

impl Generator for Illaoi {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::P, [(0, Void) /* Description 1 */])
            .modify(P(Void), |_| {
                f![9 + 171 / 17 * (Level - 1) + 1.1 * AttackDamage + 0.4 * AbilityPower]
            })?
            .comment(P(Void), "Tentacle slam")?
            .clone_with(P(Void), Q(Void), |p| f![(p) * (1.1 + 0.05 * (QLevel - 1))])?
            .comment(Q(Void), "Tentacle slam increased by Tentacle Smash's rank")?
            .ability(
                Key::W,
                [
                    (0, Max), /* Additional Physical Damage */
                    (1, Min), /* Minimum Physical Damage */
                ],
            )
            .modify(W(Max), |_| {
                f![(0.03 + 0.005 * (WLevel - 1) + 0.00035 * AttackDamage) * EnemyMaxHealth]
            })?
            .ability(Key::R, [(0, Void) /* Physical Damage */])
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Min)),
                    Attack,
                    Ability(W(Min)),
                ],
                &[
                    Attack,
                    Ability(W(Min)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(W(Min)),
                    Attack,
                    Ability(W(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(W(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Min)),
                ],
                &[
                    Ability(W(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(W(Min)),
                ],
                &[Attack, Ability(W(Max)), Ability(Q(Void)), Attack, Attack],
                &[Ability(W(Max)), Attack, Ability(Q(Void)), Attack],
                &[Attack, Ability(W(Max)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
