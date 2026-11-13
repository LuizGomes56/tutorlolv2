use super::*;

impl Generator for Syndra {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Bonus True Damage", W(Void))?
            .take("Magic Damage", W(Min))?
            .take("Total Mixed Damage", W(Max))?;

        let bonus = self.damage(W(Void))?;

        self.edit(W(Max), |w| f![w * MagicMultiplier + bonus])
            .take("Magic Damage", E(Void))?
            .take("Magic Damage per Sphere", R(Void))?
            .take("Maximum Magic Damage", R(Max))?
            .take("Minimum Magic Damage", R(Min))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Ability(R(Void)), Ability(E(Void))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                ],
                &[
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                ],
            ])?
            .end()
    }
}
