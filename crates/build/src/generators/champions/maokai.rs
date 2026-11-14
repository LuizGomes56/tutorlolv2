use super::*;

impl Generator for Maokai {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .map_change(Q(Void), "AP ratio")?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", E(Min))?
            .map_change(E(Min), "AP ratio")?
            .take("Non-Minion Total Magic Damage", E(Max))?
            .map_change(E(Max), "Total enhanced AP ratio")?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(Q(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(R(Void)),
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(W(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
            ])?
            .end()
    }
}
