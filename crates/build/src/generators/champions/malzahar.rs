use super::*;

impl Generator for Malzahar {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Magic Damage", W(Void))?
            .map_change(W(Void), "Voidling bonus AD ratio")?
            .take("Magic Damage Per Tick", E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take_nth("Magic Damage Per Tick", 1, R(Min))?
            .take_nth("Total Magic Damage", 1, R(Max))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Ability(R(Max)),
                ],
                &[
                    Attack,
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(R(Max)),
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Ability(W(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                    Ability(R(Max)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                ],
                &[Ability(E(Max)), Ability(Q(Void)), Ability(W(Void)), Attack],
                &[
                    Ability(E(Max)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(R(Max)),
                ],
            ])?
            .end()
    }
}
