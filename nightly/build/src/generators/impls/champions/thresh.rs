use super::*;

impl Generator for Thresh {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(
                Key::E,
                [
                    ("Magic Damage", Void),
                    ("Maximum Bonus Magic Damage", _1Max),
                    ("Minimum Bonus Magic Damage", _1Min),
                ],
            )
            .ability(Key::R, [("Magic Damage", Void)])
            .combos([
                &[
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Ability(R(Void)),
                    Attack,
                ],
                &[Ability(Q(Void)), Ability(Q(Void)), Ability(E(Void)), Attack],
                &[Ability(Q(Void)), Ability(R(Void)), Ability(E(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Ability(R(Void)), Attack],
                &[Ability(R(Void)), Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Ability(E(Void)), Ability(Q(Void)), Attack],
                &[Ability(Q(Void)), Ability(E(Void)), Attack],
                &[
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Ability(Q(Void)),
                    Attack,
                ],
                &[Attack, Ability(E(Void)), Attack],
            ])?
            .end()
    }
}
