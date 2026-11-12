use super::*;

impl GeneratorV2 for Talon {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Physical Damage", Q(Min))?
            .take("Critical Physical Damage", Q(Max))?
            .take_nth("Physical Damage", 1, W(Min))?
            .take("Total Physical Damage", W(Max))?
            .take("Physical Damage", R(Min))?
            .take("Total Physical Damage", R(Max))?
            .combos([
                &[Ability(W(Max)), Ability(Q(Max)), Ability(R(Max)), Attack],
                &[
                    Ability(W(Max)),
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(Q(Min)),
                    Attack,
                ],
                &[Ability(W(Max)), Ability(R(Max)), Ability(Q(Max)), Attack],
                &[Ability(W(Max)), Attack, Ability(Q(Max)), Attack],
                &[Ability(W(Max)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
