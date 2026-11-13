use super::*;

impl GeneratorV2 for Sylas {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Unshackled", 1, P(Min))?
            .take_nth("Unshackled", 0, P(Max))?
            .take_nth("Magic Damage", 1, Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Void))?
            .comment(P(Min), "Secondary Target Damage")
            .comment(P(Max), "Primary Target Damage")
            .combos([
                &[Ability(Q(Max)), Ability(W(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(E(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
