use super::*;

impl Generator for Sejuani {
    fn generate(&mut self) -> MayFail {
        self.take("Innate - Icebreaker", P(Void))?
            .take("Magic Damage", Q(Void))?
            .take_nth("Physical Damage", 1, W(Min))?
            .take("Total Physical Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Min))?
            .take("Increased Damage", R(Max))?
            .combos([
                &[
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(R(Max)), Ability(W(Max)), Attack],
                &[
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                ],
                &[Ability(R(Max)), Ability(W(Max)), Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
