use super::*;

impl Generator for TwistedFate {
    fn generate(&mut self) -> MayFail {
        self.ability(Key::Q, [(0, Void) /* Magic Damage */])
            .ability(Key::W, [(0, Void) /* Magic Damage */])
            .ability(Key::E, [(1, Void) /* Bonus Magic Damage */])
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
