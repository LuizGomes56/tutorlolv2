use super::*;

impl Generator for Neeko {
    fn generate(&mut self) -> MayFail {
        self.take("Subsequent Magic Damage", Q(_1))?
            .take("Initial Magic Damage", Q(Min))?
            .take("Total Maximum Magic Damage", Q(Max))?
            .take("Bonus Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
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
