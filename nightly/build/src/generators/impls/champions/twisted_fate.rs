use super::*;

impl Generator for TwistedFate {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [("Magic Damage", Void)])
            .ability(Key::W, [("Magic Damage", Void)])
            .ability(Key::E, [("Bonus Magic Damage", Void)])
            .combos([
                &[Ability(W(Void)), Ability(W(Void)), Ability(E(Void)), Attack],
                &[
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(Q(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                    Attack,
                ],
                &[
                    Attack,
                    Ability(W(Void)),
                    Attack,
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                ],
                &[
                    Ability(W(Void)),
                    Ability(W(Void)),
                    Ability(E(Void)),
                    Attack,
                    Ability(Q(Void)),
                ],
            ])?
            .end()
    }
}
