use super::*;

impl Generator for Caitlyn {
    fn generate(&mut self) -> MayFail {
        self.take("Headshot", P(Void))?
            .comment(P(Void), "Headshot bonus physical damage")
            .take("Physical Damage", Q(Max))?
            .take("Reduced Damage", Q(Min))?
            .take("Headshot Damage Increase", W(Void))?
            .damage_type(W(Void), Physical)
            .comment(W(Void), "Bonus Headshot damage against trapped enemies")
            .take("Magic Damage", E(Void))?
            .take("Physical damage", R(Void))?
            .combos([
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Attack],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Max)), Attack],
                &[
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack, Attack],
                &[
                    Ability(E(Void)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Max)),
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Ability(R(Void))],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Max)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(E(Void)), Ability(Q(Max)), Attack],
            ])?
            .end()
    }
}
