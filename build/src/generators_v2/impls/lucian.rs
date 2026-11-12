use super::*;

impl GeneratorV2 for Lucian {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            // 50 / 55 / 60% of AD, whose `% AD` the model drops
            .edit(P(Void), |p| f![p / 100 * AttackDamage])
            .insert(
                P(_1),
                Damage {
                    formula: f![5 + 0.15 * AttackDamage],
                    name: "Lightslinger".into(),
                    damage_type: Magic,
                    comment: "Innate - Vigilance".into(),
                    attributes: Undefined,
                },
            )?
            .take("Physical Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Physical Damage Per Shot", R(Min))?
            .derive(R(Max), R(Min), |dmg| {
                let shots = f![2.2222 * CritChance / 100];
                /* Maybe include IE bonus shots in the future */
                f![(dmg) * shots]
            })?
            .comment(R(Max), "Maximum Damage without IE")
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Attack, Ability(Q(Void)), Attack],
                &[
                    Ability(W(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Attack, Ability(W(Void)), Attack],
                &[Attack, Ability(Q(Void)), Attack, Ability(W(Void)), Attack],
                &[Ability(Q(Void)), Attack, Attack],
            ])?
            .end()
    }
}
