use super::*;

impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.take("Subsequent Magic Damage", Q(_1))?
            .map_change(Q(_1), "Second and third hit base damage")?
            .take("Initial Magic Damage", Q(Min))?
            .map_change(Q(Min), "Initial hit base damage")?
            .take("Total Maximum Magic Damage", Q(Max))?
            .take("Bonus Magic Damage", W(Void))?
            .map_change(W(Void), "On-hit base damage")?
            .map_change(W(Void), "On-hit AP ratio")?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?;

        let initial = self.damage(Q(Min))?;
        let subsequent = self.damage(Q(_1))?;

        self.set(Q(Max), initial + subsequent.scale(2.))
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(R(Void)),
                ],
                &[Ability(Q(_1)), Attack, Ability(W(Void))],
            ])?
            .end()
    }
}
