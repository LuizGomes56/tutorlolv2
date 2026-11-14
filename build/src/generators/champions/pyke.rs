use super::*;

impl Generator for Pyke {
    fn generate(&mut self) -> MayFail {
        self.take("Physical Damage", Q(Void))?
            .take("Physical Damage", E(Void))?
            .take_nth("Active", 1, R(Void))?
            .take_nth("Active", 0, R(_1))?
            .damage_type(R(Void), Physical)
            .damage_type(R(_1), True)
            .combos([
                &[
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Attack,
                ],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void))],
                &[Ability(E(Void)), Ability(R(Void))],
                &[Ability(Q(Void)), Ability(E(Void)), Ability(R(Void))],
                &[Ability(E(Void)), Ability(Q(Void)), Attack, Ability(R(Void))],
                &[Ability(Q(Void)), Ability(R(Void)), Ability(E(Void))],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                    Attack,
                    Ability(R(Void)),
                ],
                &[Ability(Q(Void)), Attack, Ability(E(Void)), Attack, Attack],
                &[Ability(Q(Void)), Ability(E(Void)), Attack, Attack, Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack, Attack, Attack],
            ])?
            .end()
    }
}
