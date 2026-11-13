use super::*;

impl Generator for Blitzcrank {
    fn generate(&mut self) -> MayFail {
        self.take("Magic Damage", Q(Void))?
            .take("Active", E(Void))?
            .damage_type(E(Void), Physical)
            .take_nth("Magic Damage", 1, R(Void))?
            .take_nth("Magic Damage", 0, R(_1))?
            .comment(R(_1), "R Passive Onhit Damage")
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(R(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(R(Void)), Attack, Ability(E(Void)), Ability(Q(Void))],
                &[
                    Ability(Q(Void)),
                    Attack,
                    Ability(E(Void)),
                    Attack,
                    Ability(R(Void)),
                ],
                &[
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Attack, Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
