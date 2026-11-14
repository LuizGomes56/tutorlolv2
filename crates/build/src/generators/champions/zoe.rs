use super::*;

impl Generator for Zoe {
    fn generate(&mut self) -> MayFail {
        self.take("Innate", P(Void))?
            .take("Maximum Magic Damage", Q(Max))?
            .take("Minimum Magic Damage", Q(Min))?
            .take("Magic Damage Per Bolt", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Magic Damage", E(Min))?
            .derive(E(Max), E(Min), |dmg| f![((dmg) * MagicMultiplier) + dmg])?
            .combos([
                &[Ability(E(Max)), Ability(Q(Min)), Ability(Q(Min))],
                &[Ability(Q(Min)), Ability(Q(Min)), Attack],
                &[Ability(Q(Min)), Ability(Q(Min))],
                &[Ability(Q(Min)), Attack, Ability(Q(Min)), Attack],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Ability(Q(Min)),
                    Attack,
                ],
            ])?
            .end()
    }
}
