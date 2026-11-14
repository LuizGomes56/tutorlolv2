use super::*;

impl Generator for Renata {
    fn generate(&mut self) -> MayFail {
        self.take_nth("Innate", 0, P(Void))?
            .take("Magic Damage", Q(Void))?
            .take("Magic Damage", E(Void))?
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(Q(Void)), Ability(E(Void))],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(Q(Void)), Attack],
                &[
                    Attack,
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(Q(Void)), Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
