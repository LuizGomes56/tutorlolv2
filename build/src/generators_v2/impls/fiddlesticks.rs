use super::*;

impl GeneratorV2 for Fiddlesticks {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .take("Increased Magic Damage", Q(Max))?
            .comment(Q(Max), "Against feared or recently feared enemies")
            .take("Damage per Instance", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Last Tick of Damage", W(_1))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage per Tick", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                ],
                &[Ability(Q(Max)), Ability(W(Max)), Ability(E(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Ability(W(Max)),
                ],
                &[Ability(Q(Max)), Ability(E(Void)), Ability(W(Max))],
                &[Ability(E(Void)), Ability(Q(Max)), Ability(W(Max))],
                &[
                    Ability(R(Max)),
                    Ability(Q(Max)),
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
