use super::*;

impl Generator for Sivir {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Void))?
            .take("Minimum Damage", Q(Min))?
            .take("Total Maximum Champion Damage", Q(Max))?
            .take("Bounce Damage", W(Min))?
            .take("Bounce Critical Damage", W(Max))?
            .combos([
                &[Ability(Q(Void)), Attack, Ability(W(Max)), Attack, Attack],
                &[Attack, Ability(W(Max)), Ability(Q(Void)), Attack, Attack],
                &[
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Attack, Ability(W(Max)), Ability(Q(Void))],
                &[Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
