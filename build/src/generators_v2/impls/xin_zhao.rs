use super::*;

impl GeneratorV2 for XinZhao {
    fn generate(&mut self) -> MayFail {
        // 15% / 30% / 45% / 60% AD (+ 5% / 10% / 15% / 20% AP) at levels 1 / 6 / 11 / 16
        let tier = Formula::Raw(
            stringify!(match ctx.level {
                ..6 => 1,
                6..11 => 2,
                11..16 => 3,
                16.. => 4,
            })
            .into(),
        );

        self.take_nth("Innate", 0, P(Void))?
            .set(P(Void), {
                f![(tier) * (0.15 * AttackDamage + 0.05 * AbilityPower)]
            })
            .comment(P(Void), "Determination third stack bonus damage")
            .take("Bonus Physical Damage", Q(Min))?
            .take("Total Bonus Physical Damage", Q(Max))?
            .take("Slash Total Physical Damage", W(Min))?
            .take("Total Physical Damage", W(Max))?
            .take("Thrust Physical Damage", W(_1))?
            .take("Magic Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(R(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
