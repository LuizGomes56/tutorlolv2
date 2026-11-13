use super::*;

impl GeneratorV2 for TahmKench {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Magic Damage", Q(Min))?
            // The model already includes An Acquired Taste in Q.
            .derive(Q(Max), Q(Min), |q| q)?
            .take("Magic Damage", W(Void))?
            .take("Magic Damage", R(Void))?
            .comment(Q(Max), "Damage Including Passive")
            .combos([
                &[Ability(R(Void)), Ability(W(Void))],
                &[
                    Ability(W(Void)),
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                ],
                &[
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(R(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Attack, Attack, Attack, Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
