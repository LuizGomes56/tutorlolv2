use super::*;

impl GeneratorV2 for Akshan {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 3, P(Void))?
            .damage_type(P(Void), Magic)
            .comment(P(Void), "Dirty Fighting third stack bonus magic damage")
            .take("Physical Damage", Q(Min))?
            .take("Total Physical Damage", Q(Max))?
            .take("Physical Damage per Shot", E(Void))?
            .take("Damage to target on 67% missing hp", R(_1Max))?
            .take("Maximum Physical Damage per Bullet", R(Max))?
            .take("Minimum Charged Physical Damage", R(_1Min))?
            .take("Minimum Physical Damage per Bullet", R(Min))?
            .combos([
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                ],
                &[Attack, Attack, Ability(Q(Max))],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                ],
            ])?
            .end()
    }
}
