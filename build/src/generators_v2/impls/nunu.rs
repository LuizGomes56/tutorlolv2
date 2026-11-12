use super::*;

impl GeneratorV2 for Nunu {
    fn generate(&mut self) -> MayFail {
        self.take("Champion Magic Damage", Q(Void))?
            .take("Minimum Magic Damage", W(Min))?
            .take("Maximum Magic Damage", W(Max))?
            .take("Magic Damage", E(_1))?
            .take("Total Magic Damage", E(_2))?
            .take("Magic Damage Per Hit", E(Min))?
            .take("Maximum Total Magic Damage", E(Max))?
            .take("Maximum Magic Damage", R(Void))?
            .damage_type(Q(Void), Magic)
            .combos([
                &[
                    Ability(W(Min)),
                    Ability(W(Min)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Attack,
                ],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(E(Max)), Ability(R(Void)), Ability(Q(Void)), Attack],
                &[
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Ability(R(Void)),
                ],
                &[
                    Ability(W(Max)),
                    Ability(E(Max)),
                    Ability(Q(Void)),
                    Ability(E(Max)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(E(Max)),
                    Ability(E(Max)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Max)),
                    Attack,
                    Ability(W(Max)),
                    Attack,
                ],
            ])?
            .end()
    }
}
