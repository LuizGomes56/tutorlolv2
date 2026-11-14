use super::*;

impl Generator for Lulu {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 1, P(Void))?
            .comment(P(Void), "Pix's 3 bolts total bonus magic damage")
            .take("Magic Damage", Q(Min))?
            .take("Total Magic Damage", Q(Max))?
            .take("Reduced Damage", Q(_1))?
            .take("Magic Damage", E(Void))?
            .combos([
                &[Attack, Ability(E(Void)), Attack, Ability(Q(Min)), Attack],
                &[Ability(E(Void)), Ability(Q(Min))],
                &[
                    Ability(Q(Min)),
                    Attack,
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Attack,
                    Ability(Q(Min)),
                ],
                &[Ability(Q(Min)), Attack, Ability(E(Void))],
                &[Ability(E(Void)), Attack, Ability(Q(Min))],
                &[Ability(E(Void)), Ability(Q(Min)), Attack, Attack],
                &[Ability(Q(Min)), Ability(E(Void)), Attack],
                &[Ability(E(Void)), Attack, Ability(Q(Min)), Attack],
            ])?
            .end()
    }
}
