use super::*;

impl GeneratorV2 for Lillia {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .set(P(Void), {
                f![(0.05 + 0.000125 * AbilityPower) * EnemyMaxHealth]
            })
            .comment(P(Void), "Dream Dust total magic damage over 3 seconds")
            .take("Magic Damage", Q(Min))?
            .take("Total Mixed Damage", Q(Max))?
            .damage_type(Q(Min), Magic)
            .take("Magic Damage", W(Min))?
            .take("Increased Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Max)),
                ],
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(W(Max)),
                    Ability(Q(Max)),
                ],
                &[
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(E(Void)), Ability(W(Max)), Ability(Q(Max))],
            ])?
            .end()
    }
}
