use super::*;

impl Generator for Nidalee {
    fn generate(&mut self) -> MayFail {
        self.take_in("Javelin Toss", "Minimum Magic Damage", Q(Min))?
            .take_in("Takedown", "Minimum Magic Damage", Q(_1Min))?
            .take_in("Javelin Toss", "Maximum Magic Damage", Q(Max))?
            .take_in("Takedown", "Maximum Magic Damage", Q(_1Max))?
            .take("Total Magic Damage", W(Void))?
            .take("Magic Damage Per Tick", W(_1))?
            .take("Magic Damage", W(_2))?
            .take("Magic Damage", E(Void))?
            .combos([
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Ability(Q(Max)), Ability(W(Void))],
                &[Ability(W(Void)), Ability(Q(Max)), Attack],
                &[Ability(E(Void)), Ability(W(Void)), Ability(Q(Max)), Attack],
                &[
                    Ability(Q(Min)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(W(Void)),
                    Ability(Q(Max)),
                    Ability(E(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
