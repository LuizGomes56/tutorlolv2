use super::*;

impl GeneratorV2 for Alistar {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage Per Tick", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void))],
                &[Ability(Q(Void)), Ability(W(Void))],
                &[
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                ],
                &[Ability(W(Void)), Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
