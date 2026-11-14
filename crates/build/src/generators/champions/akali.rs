use super::*;

impl Generator for Akali {
    fn generate(&mut self) -> MayFail {
        self.take("Swinging Kama", P(Void))?
            .take("Magic Damage", Q(Void))?
            .take_nth("Magic Damage", 1, E(Min))?
            .take("Total Magic Damage", E(Max))?
            .take("Magic Damage", R(_1))?
            .take("Maximum Magic Damage", R(Max))?
            .take("Minimum Magic Damage", R(Min))?
            .combos([
                &[
                    Ability(R(_1)),
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Min)),
                ],
                &[
                    Ability(R(_1)),
                    Ability(E(Min)),
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(R(Min)),
                ],
                &[
                    Ability(R(_1)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Min)),
                ],
                &[Ability(E(Min)), Ability(E(Min)), Ability(Q(Void)), Attack],
                &[
                    Ability(R(_1)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(R(Min)),
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                ],
                &[
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Min)),
                    Ability(E(Min)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
            ])?
            .end()
    }
}
