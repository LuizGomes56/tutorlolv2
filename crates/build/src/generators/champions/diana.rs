use super::*;

impl Generator for Diana {
    fn generate(&mut self) -> MayFail {
        self.take("Innate - Moonsilver Blade", P(Void))?
            .take("Magic Damage", Q(_1))?
            .take("Magic Damage per Orb", W(Min))?
            .take("Total Magic Damage", W(Max))?
            .take("Magic Damage", E(Void))?
            .take("Bonus Damage Per Champion", R(_1))?
            .take("Magic Damage", R(Min))?
            .take("Total Damage Vs. 5 Champions", R(Max))?
            .combos([
                &[
                    Ability(R(Max)),
                    Ability(Q(_1)),
                    Ability(W(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(_1)),
                    Attack,
                ],
                &[
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(W(Max)), Ability(E(Void)), Ability(Q(_1))],
                &[
                    Ability(E(Void)),
                    Ability(R(Max)),
                    Ability(W(Max)),
                    Ability(Q(_1)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Ability(W(Max)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Max)),
                    Attack,
                    Attack,
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Attack,
                    Ability(Q(_1)),
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Void)),
                ],
                &[Attack, Ability(W(Max)), Ability(Q(_1)), Attack, Attack],
            ])?
            .end()
    }
}
