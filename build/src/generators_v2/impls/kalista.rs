use super::*;

impl GeneratorV2 for Kalista {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Void))?
            .take("Bonus Magic Damage", W(Void))?
            .take("Bonus Damage per Additional Stack", E(_1))?
            .take("Physical Damage", E(Void))?;

        let e1 = self.damage(E(_1))?;

        // Each additional spear deals the bonus of E(_1)
        self.edit(E(Void), |e| f![(e) + ((e1) * Stacks)])
            .combos([
                &[
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                ],
                &[Attack, Ability(Q(Void)), Ability(E(Void))],
                &[Ability(Q(Void)), Attack, Ability(E(Void))],
                &[Attack, Ability(Q(Void)), Attack, Ability(E(Void))],
            ])?
            .end()
    }
}
