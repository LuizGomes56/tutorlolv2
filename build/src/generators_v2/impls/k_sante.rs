use super::*;

impl GeneratorV2 for KSante {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .set(P(Void), {
                f![12 + (0.01 + 0.01 / 17 * (Level - 1)) * EnemyMaxHealth]
            })
            .damage_type(P(Void), Physical)
            .comment(P(Void), "Mark consumption bonus damage")
            .take("Physical Damage", Q(Void))?
            .take("Physical Damage", W(Min))?
            .take("Total Maximum Mixed Damage", W(Max))?
            .set(W(Min), {
                f![45
                    + 30 * (WLevel - 1)
                    + (0.08 + 0.0002 * BonusArmor + 0.0002 * BonusMagicResist) * EnemyMaxHealth]
            })
            .set(W(Max), {
                f![81
                    + 54 * (WLevel - 1)
                    + (0.144 + 0.00036 * BonusArmor + 0.00036 * BonusMagicResist) * EnemyMaxHealth]
            })
            .damage_type(W(Max), Mixed)
            .take("Strike Physical Damage", R(Min))?
            .take("Total Physical Damage", R(Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
