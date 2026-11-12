use super::*;

impl GeneratorV2 for Caitlyn {
    fn generate(&mut self) -> MayFail {
        // 60% / 70% / 80% AD at levels 1 / 7 / 13
        let headshot = Formula::Raw(
            stringify!(match ctx.level {
                ..7 => 0.6,
                7..13 => 0.7,
                13.. => 0.8,
            })
            .into(),
        );

        self.take("Headshot", P(Void))?
            .set(P(Void), {
                f![((headshot) + 0.8 * CritChance / 100) * AttackDamage]
            })
            .comment(P(Void), "Headshot bonus physical damage")
            .take("Physical Damage", Q(Max))?
            .take("Reduced Damage", Q(Min))?
            .take("Headshot Damage Increase", W(Void))?
            .damage_type(W(Void), Physical)
            .comment(W(Void), "Bonus Headshot damage against trapped enemies")
            .take("Magic Damage", E(Void))?
            .take("Physical damage", R(Void))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Attack],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
