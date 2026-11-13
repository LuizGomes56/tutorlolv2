use super::*;

impl Generator for XinZhao {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .comment(P(Void), "Determination third stack bonus damage")
            .take("Bonus Physical Damage", Q(Min))?
            .take("Total Bonus Physical Damage", Q(Max))?
            .take("Slash Total Physical Damage", W(Min))?
            .take("Total Physical Damage", W(Max))?
            .take("Thrust Physical Damage", W(_1))?
            .take("Magic Damage", E(Void))?
            .take("Physical Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Ability(R(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
