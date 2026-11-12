use super::*;

impl GeneratorV2 for Vladimir {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Min))?
            .take("Increased Damage", Q(Max))?
            .take("Magic Damage Per Tick", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Minimum Magic Damage", E(Min))?
            .take("Maximum Magic Damage", E(Max))?
            .take("Magic damage", R(Void))?
            .combos([
                &[
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Ability(W(Max)),
                ],
                &[
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Ability(W(Max)),
                ],
                &[Ability(E(Max)), Ability(W(Max)), Ability(Q(Max))],
                &[Ability(E(Max)), Ability(Q(Max)), Ability(W(Max))],
                &[Ability(E(Max)), Ability(Q(Max))],
                &[
                    Ability(E(Max)),
                    Ability(R(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Max)),
                ],
            ])?
            .end()
    }
}
