use super::*;

impl GeneratorV2 for Veigar {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Minimum Magic Damage", R(Min))?
            .take("Maximum Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Max))],
                &[Ability(Q(Void)), Ability(R(Max))],
                &[Ability(W(Void)), Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
