use super::*;

impl GeneratorV2 for Riven {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .set(P(Void), {
                f![(30 + 15 / 17 * (Level - 1)) / 100 * AttackDamage]
            })
            .take("Physical Damage", Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Physical Damage", W(Void))?
            .take("Maximum Physical Damage", R(Max))?
            .take("Minimum Physical Damage", R(Min))?
            .damage_type(P(Void), Physical)
            .damage_type(Q(Min), Physical)
            .damage_type(Q(Max), Physical)
            .damage_type(W(Void), Physical)
            .damage_type(R(Min), Physical)
            .damage_type(R(Max), Physical)
            .combos([
                &[
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[
                    Ability(R(Min)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Min)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Ability(R(Min)),
                    Ability(Q(Min)),
                    Attack,
                    Ability(R(Min)),
                    Ability(W(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Void)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Max))],
                &[Ability(R(Max)), Ability(Q(Max))],
                &[Ability(R(Max)), Ability(W(Void))],
            ])?
            .end()
    }
}
