use super::*;

impl GeneratorV2 for Gangplank {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .comment(P(Void), "Trial by Fire true damage over 2.5 seconds")
            .take("Physical Damage", Q(Void))?
            .take("Bonus Champion Damage", E(Void))?
            .take("Magic Damage Per Wave", R(Min))?
            .take("Total Magic Damage", R(Max))?
            .take("Total Magic Damage with Fire at Will", R(_1))?
            .take("Magic Damage Per Cluster", R(_2))?
            .damage_type(R(_2), True)
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                ],
                &[Ability(E(Void)), Attack, Ability(E(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(E(Void))],
                &[
                    Ability(R(Max)),
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                ],
                &[Ability(Q(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(E(Void)), Attack, Attack, Ability(Q(Void)), Attack],
            ])?
            .end()
    }
}
