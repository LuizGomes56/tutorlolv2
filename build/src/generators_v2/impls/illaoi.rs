use super::*;

impl GeneratorV2 for Illaoi {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .set(P(Void), {
                f![9 + 171 / 17 * (Level - 1) + 1.1 * AttackDamage + 0.4 * AbilityPower]
            })
            .comment(P(Void), "Tentacle slam")
            .derive(Q(Void), P(Void), |p| f![(p) * (1.1 + 0.05 * (QLevel - 1))])?
            .comment(Q(Void), "Tentacle slam increased by Tentacle Smash's rank")
            .take("Additional Physical Damage", W(Max))?
            .take("Minimum Physical Damage", W(Min))?
            .set(W(Max), {
                f![(0.03 + 0.005 * (WLevel - 1) + 0.00035 * AttackDamage) * EnemyMaxHealth]
            })
            .take("Physical Damage", R(Void))?
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
