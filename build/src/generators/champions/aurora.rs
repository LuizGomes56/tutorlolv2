use super::*;

impl Generator for Aurora {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .comment(P(Void), "Third stack bonus magic damage")
            .take("Maximum Magic Damage", Q(Max))?
            .take("Minimum Magic Damage", Q(Min))?
            .take("Subsequent Bolt Maximum Magic Damage", Q(_1Max))?
            .take("Subsequent Bolt Minimum Magic Damage", Q(_1Min))?
            .take("Magic Damage", E(Void))?
            .take("Magic Damage", R(Void))?
            .combos([
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Ability(Q(Min)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Min)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Min)), Ability(E(Void)), Ability(Q(Min))],
                &[Ability(Q(Min)), Attack, Ability(E(Void)), Ability(Q(Min))],
            ])?
            .end()
    }
}
